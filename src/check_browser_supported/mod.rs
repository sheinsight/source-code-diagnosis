mod classes;
mod compat;
mod functions;
mod grammar;
mod macros;
mod operators;
mod statements;
use std::{
  fs::read_to_string,
  path::PathBuf,
  sync::{Arc, Mutex},
};

use browserslist::{resolve, Distrib, Opts};
use compat::CompatBox;
use napi::{Error, Result};

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

#[napi]
pub fn check_browser_supported(
  target: String,
  options: Option<Options>,
) -> Result<Vec<CompatBox>> {
  let browser_list = resolve(&[target], &Opts::default())
    .map_err(|err| Error::new(napi::Status::GenericFailure, err.to_string()))?;

  let chrome_version_list = get_version_list(&browser_list, "chrome");
  let firefox_version_list = get_version_list(&browser_list, "firefox");
  let edge_version_list = get_version_list(&browser_list, "edge");
  let safari_version_list = get_version_list(&browser_list, "safari");
  let node_version_list = get_version_list(&browser_list, "node");

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
            let mut used = used.lock().unwrap();
            used.push(CompatBox::new(
              node.kind().span(),
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
