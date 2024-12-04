use anyhow::Result;
use beans::AstNode;
use napi_derive::napi;
use oxc_ast::AstKind;
use serde::Serialize;
use utils::{glob_by, GlobArgs, SemanticBuilder};

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct CheckDangerResponse {
  pub raw_value: String,
  pub match_danger_string: String,
  pub file_path: String,
  pub ast_node: AstNode,
}

pub fn check_danger_strings(
  danger_strings: Vec<String>,
  args: GlobArgs,
) -> Result<Vec<CheckDangerResponse>> {
  let responses = glob_by(
    |path| {
      let mut inline_usages: Vec<CheckDangerResponse> = Vec::new();

      let builder = SemanticBuilder::with_file(path).unwrap();

      let semantic = builder.build().unwrap();

      for node in semantic.nodes().iter() {
        if let AstKind::StringLiteral(string_literal) = node.kind() {
          let value = string_literal.value.to_string();

          let ast_node = AstNode::with_source_and_span(
            semantic.source_text(),
            &string_literal.span,
          );

          for danger_string in danger_strings.iter() {
            if value.contains(danger_string) {
              inline_usages.push(CheckDangerResponse {
                raw_value: value.clone(),
                match_danger_string: danger_string.to_string(),
                file_path: path.display().to_string(),
                ast_node,
              });
            }
          }
        }
      }

      Some(inline_usages)
    },
    &args,
  )?
  .into_iter()
  .flatten()
  .collect();

  Ok(responses)
}
