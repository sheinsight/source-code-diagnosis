mod classes;
mod compat;
mod functions;
mod grammar;
mod macros;
mod offset_to_position;
mod operators;
mod statements;
use browserslist::{resolve, Distrib, Opts};
use compat::{CompatBox, Position};
use env_logger::Env;
use log::debug;
use napi::{Error, Result};
use std::{
  fs::read_to_string,
  path::PathBuf,
  sync::{Arc, Mutex},
};

use oxc_allocator::Allocator;

use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::GetSpan;
use oxc_span::SourceType;

use crate::{
  check_browser_supported::compat::CompatHandler,
  oxc_visitor_processor::{oxc_visit_process, Options},
};

fn get_version_list<'a>(
  browser_list: &'a Vec<Distrib>,
  name: &str,
) -> Vec<&'a str> {
  browser_list
    .iter()
    .filter(|x| x.name() == name)
    .map(|x| x.version())
    .collect()
}

macro_rules! enabled_debug {
  ($($body:tt)*) => {
      if log::log_enabled!(log::Level::Debug) {
          $($body)*
      }
  };
}

#[napi]
pub fn check_browser_supported(
  target: String,
  options: Option<Options>,
) -> Result<Vec<CompatBox>> {
  let env =
    Env::default().filter_or("SHINED_SOURCE_CODE_DIAGNOSIS_LOG", "info");

  env_logger::init_from_env(env);

  debug!("User-specified browser target: {}", target);

  let browser_list = resolve(&[target], &Opts::default())
    .map_err(|err| Error::new(napi::Status::GenericFailure, err.to_string()))?;

  let chrome_version_list = get_version_list(&browser_list, "chrome");

  enabled_debug! {
    for version in chrome_version_list.iter() {
      debug!("Resolved Chrome version: {}", version);
    }
  }

  let firefox_version_list = get_version_list(&browser_list, "firefox");
  enabled_debug! {
    for version in firefox_version_list.iter() {
      debug!("Resolved Firefox versions: {:?}", version);
    }
  }

  let edge_version_list = get_version_list(&browser_list, "edge");

  enabled_debug! {
    for version in edge_version_list.iter() {
      debug!("Resolved Edge versions: {:?}", version);
    }
  }

  let safari_version_list = get_version_list(&browser_list, "safari");

  enabled_debug! {
    for version in safari_version_list.iter() {
      debug!("Resolved Safari versions: {:?}", version);
    }
  }

  let node_version_list = get_version_list(&browser_list, "node");

  enabled_debug! {
    for version in node_version_list.iter() {
      debug!("Resolved Node versions: {:?}", version);
    }
  }

  let compat_handlers: Vec<Box<dyn CompatHandler>> = vec![
    classes::setup(),
    functions::setup(),
    grammar::setup(),
    operators::setup(),
    statements::setup(),
  ]
  .into_iter()
  .flat_map(|setup| setup.into_iter())
  .filter(|item| {
    let compat = item.get_compat();
    let compat_support = &compat.support;
    return browser_list.iter().any(|x| match x.name() {
      "chrome" => chrome_version_list.contains(&compat_support.chrome.as_str()),
      "firefox" => {
        firefox_version_list.contains(&compat_support.firefox.as_str())
      }
      "edge" => edge_version_list.contains(&compat_support.edge.as_str()),
      "safari" => safari_version_list.contains(&compat_support.safari.as_str()),
      "node" => node_version_list.contains(&compat_support.node.as_str()),
      _ => true,
    });
  })
  .collect();

  enabled_debug! {
    for compat_handler in compat_handlers.iter() {
      debug!(
        "Compat handler: {:?}",
        compat_handler.get_compat().name.clone()
      );
    }
  }

  let share = Arc::new(compat_handlers);
  let used: Arc<Mutex<Vec<CompatBox>>> = Arc::new(Mutex::new(Vec::new()));
  let handler = {
    let used = Arc::clone(&used);
    let clone = Arc::clone(&share);
    move |path: PathBuf| {
      let source_code = read_to_string(&path)
        .map_err(|err| {
          Error::new(
            napi::Status::GenericFailure,
            format!("Failed to read file: {}: {}", path.display(), err),
          )
        })
        .unwrap();

      let source_type = SourceType::from_path(&path)
        .map_err(|e| Error::new(napi::Status::GenericFailure, e.0.to_string()))
        .unwrap();

      let allocator = Allocator::default();
      let ret = Parser::new(&allocator, &source_code, source_type).parse();
      let program = allocator.alloc(ret.program);

      let semantic = SemanticBuilder::new(&source_code, source_type)
        .build(program)
        .semantic;

      let nodes = semantic.nodes();

      for node in nodes.iter() {
        for compat_handler in clone.iter() {
          if compat_handler.handle(source_code.as_str(), node, nodes) {
            let span = oxc_span::GetSpan::span(&node.kind());
            let start_position = crate::check_browser_supported::offset_to_position::offset_to_position(span.start as usize, &source_code).unwrap();
            let end_position = crate::check_browser_supported::offset_to_position::offset_to_position(span.end as usize, &source_code).unwrap();

            let mut used = used.lock().unwrap();
            used.push(CompatBox::new(
              node.kind().span(),
              compat::Location {
                start: Position {
                  line: start_position.line,
                  column: start_position.character,
                },
                end: Position {
                  line: end_position.line,
                  column: end_position.character,
                },
              },
              compat_handler.get_compat().clone(),
              path.to_str().unwrap().to_string(),
            ));
          }
        }
      }
    }
  };

  oxc_visit_process(handler, options)?;

  let used = Arc::try_unwrap(used)
    .ok()
    .expect("Arc has more than one strong reference")
    .into_inner()
    .expect("Mutex cannot be locked");

  Ok(used)
}
