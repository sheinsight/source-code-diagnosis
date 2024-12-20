use anyhow::Result;
use beans::AstNode;
use napi_derive::napi;
use oxc_ast::AstKind;
use serde::Serialize;
use utils::{GlobArgs, GlobErrorHandler, GlobSuccessHandler};

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct CheckDangerResponse {
  pub file_path: String,
  pub items: Vec<CheckDangerResponseItem>,
  pub errors: Vec<String>,
}

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct CheckDangerResponseItem {
  pub raw_value: String,
  pub match_danger_string: String,
  pub ast_node: AstNode,
}

pub fn check_danger_strings(
  danger_strings: Vec<String>,
  args: GlobArgs,
) -> Result<Vec<CheckDangerResponse>> {
  let responses = utils::glob_by_semantic(
    |GlobSuccessHandler {
       semantic,
       relative_path,
       ..
     }| {
      let responses = semantic
        .nodes()
        .into_iter()
        .filter_map(|node| {
          if let AstKind::StringLiteral(string_literal) = node.kind() {
            let value = string_literal.value.to_string();

            let ast_node = AstNode::with_source_and_span(
              &semantic.source_text(),
              &string_literal.span,
            );

            for danger_string in danger_strings.iter() {
              if value.contains(danger_string) {
                return Some(CheckDangerResponseItem {
                  raw_value: value.clone(),
                  match_danger_string: danger_string.to_string(),
                  ast_node,
                });
              }
            }
          }
          None
        })
        .collect::<Vec<_>>();
      Some(CheckDangerResponse {
        file_path: relative_path,
        items: responses,
        errors: vec![],
      })
    },
    |GlobErrorHandler {
       relative_path,
       error,
       ..
     }| {
      return Some(CheckDangerResponse {
        file_path: relative_path,
        items: vec![],
        errors: vec![error],
      });
    },
    &args,
  )?;

  Ok(responses)
}
