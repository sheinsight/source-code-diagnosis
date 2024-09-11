use std::{
  path::PathBuf,
  sync::{Arc, Mutex},
};

use napi::Result;
use oxc_ast::AstKind;
use serde::Serialize;

use crate::{
  oxc_visitor_processor::{oxc_visit_process, Options},
  utils::{ast_node::AstNode, semantic_builder::SemanticBuilder},
};

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct Response {
  pub raw_value: String,
  pub match_danger_string: String,
  pub file_path: String,
  pub ast_node: AstNode,
}

#[napi]
pub fn get_danger_strings_usage(
  danger_strings: Vec<String>,
  options: Option<Options>,
) -> Result<Vec<Response>> {
  let used = Arc::new(Mutex::new(Vec::new()));
  let handler = {
    let used = Arc::clone(&used);
    move |path: PathBuf| {
      let mut inline_usages: Vec<Response> = Vec::new();

      SemanticBuilder::file(path.clone())
        .build_handler()
        .each_node(|handler, semantic, node| {
          if let AstKind::StringLiteral(string_literal) = node.kind() {
            let value = string_literal.value.to_string();
            let span = string_literal.span;

            let loc = handler.offset_to_location(&semantic.source_text(), span);

            danger_strings
              .iter()
              .filter(|item| value.contains(&**item))
              .for_each(|item| {
                inline_usages.push(Response {
                  raw_value: value.to_string(),
                  match_danger_string: item.to_string(),
                  file_path: path.display().to_string(),
                  ast_node: AstNode::new((span.start, span.end), loc),
                })
              })
          }
        });

      let mut used = used.lock().unwrap();
      used.extend(inline_usages);
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
