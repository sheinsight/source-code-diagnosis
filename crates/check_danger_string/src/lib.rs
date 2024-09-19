use std::{path::PathBuf, sync::Arc};

use anyhow::{Context, Result};
use beans::AstNode;
use napi_derive::napi;
use oxc_ast::AstKind;
use parking_lot::Mutex;
use serde::Serialize;
use utils::{glob, GlobOptions, SemanticBuilder};

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct Response {
  pub raw_value: String,
  pub match_danger_string: String,
  pub file_path: String,
  pub ast_node: AstNode,
}

pub fn check_danger_strings(
  danger_strings: Vec<String>,
  options: Option<GlobOptions>,
) -> Result<Vec<Response>> {
  let used = Arc::new(Mutex::new(Vec::new()));

  let handler = {
    let used = Arc::clone(&used);
    move |path: PathBuf| {
      let mut inline_usages: Vec<Response> = Vec::new();
      SemanticBuilder::file(path.clone())
        .build_handler()
        .each_node(|handler, node| {
          if let AstKind::StringLiteral(string_literal) = node.kind() {
            let value = string_literal.value.to_string();
            let span = string_literal.span;

            let loc =
              handler.offset_to_location(handler.semantic.source_text(), span);

            for danger_string in danger_strings.iter() {
              if value.contains(danger_string) {
                inline_usages.push(Response {
                  raw_value: value.clone(),
                  match_danger_string: danger_string.to_string(),
                  file_path: path.display().to_string(),
                  ast_node: AstNode::new((span.start, span.end), loc),
                });
              }
            }
          }
        });
      used.lock().extend(inline_usages);
    }
  };

  glob(handler, options)?;

  let used = Arc::try_unwrap(used)
    .ok()
    .context("Arc has more than one strong reference")?
    .into_inner();

  Ok(used)
}
