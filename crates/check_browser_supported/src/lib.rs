mod classes;
mod compat;
mod functions;
mod grammar;
mod macros;
mod operators;
mod statements;

use browserslist::{resolve, Distrib, Opts};
pub use compat::{CompatBox, CompatHandler};

use anyhow::Result;
use log::debug;
use napi::Error;
use napi_derive::napi;

use utils::{glob_by_path, SemanticBuilder};

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

#[derive(Debug, Clone)]
#[napi[object]]
pub struct Target {
  pub chrome: String,
  pub firefox: Option<String>,
  pub safari: Option<String>,
  pub edge: Option<String>,
  pub node: Option<String>,
  pub deno: Option<String>,
}

pub fn check_browser_supported_with_source_code(
  target: Target,
  source_code: String,
) -> Result<Vec<CompatBox>> {
  debug!("User-specified browser target: {:?}", target);

  let chrome_queries = format!("chrome > {}", target.chrome);

  let browser_list = resolve(&[chrome_queries], &Opts::default())
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

  let mut used: Vec<CompatBox> = Vec::new();

  let builder = SemanticBuilder::js(&source_code);

  let handler = match builder.build_handler() {
    Ok(handler) => handler,
    Err(e) => {
      eprintln!("parse error: {}", e);
      return Err(e);
    }
  };

  handler.each_node(|handler, node| {
    for compat_handler in compat_handlers.iter() {
      if compat_handler.handle(
        handler.semantic.source_text(),
        node,
        handler.semantic.nodes(),
      ) {
        let ast_node = beans::AstNode::with_source_and_ast_node(
          handler.semantic.source_text(),
          node,
        );

        used.push(CompatBox::new(
          ast_node,
          compat_handler.get_compat().clone(),
          String::new(),
        ));
      }
    }
  });

  Ok(used)
}

pub fn check_browser_supported(
  target: Target,
  args: utils::GlobArgs,
) -> Result<Vec<CompatBox>> {
  debug!("User-specified browser target: {:?}", target);

  let chrome_queries = format!("chrome > {}", target.chrome);

  let browser_list = resolve(&[chrome_queries], &Opts::default())
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

  let responses = glob_by_path(
    |path| {
      let mut used: Vec<CompatBox> = Vec::new();
      let builder = SemanticBuilder::with_file(&path).unwrap();
      let semantic = builder.build().unwrap();
      for node in semantic.nodes().iter() {
        for compat_handler in compat_handlers.iter() {
          if compat_handler.handle(
            semantic.source_text(),
            node,
            semantic.nodes(),
          ) {
            let ast_node = beans::AstNode::with_source_and_ast_node(
              semantic.source_text(),
              node,
            );

            used.push(CompatBox::new(
              ast_node,
              compat_handler.get_compat().clone(),
              path.to_str().unwrap().to_string(),
            ));
          }
        }
      }
      Some(used)
    },
    &args,
  )?
  .into_iter()
  .flatten()
  .collect();

  Ok(responses)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_check_browser_supported_with_source_code() {
    // Set up test data
    let target = "chrome >= 40".to_string();
    let source_code = r#"
            class MyClass {
                #privateField = 42;
                
                constructor() {
                    console.log(this.#privateField);
                }
            }
            
            new MyClass();
        "#
    .to_string();

    // Call the function
    let result = check_browser_supported_with_source_code(
      Target {
        chrome: "40".to_string(),
        firefox: None,
        safari: None,
        edge: None,
        node: None,
        deno: None,
      },
      source_code,
    );

    // Assert the result
    assert!(result.is_ok());
    let compat_boxes = result.unwrap();

    // Check if we have at least one CompatBox (for private class fields)
    assert!(!compat_boxes.is_empty());

    // Check the first CompatBox
    let first_compat = &compat_boxes[0];
    assert_eq!(first_compat.compat.name, "statements.classes");
    assert!(first_compat.compat.support.chrome.parse::<f32>().unwrap() > 40.0);
  }
}
