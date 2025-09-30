use std::fmt::Display;

use napi_derive::napi;
use oxc::ast::{AstKind, ast::JSXAttributeItem};
use serde::Serialize;
use utils::{GlobErrorHandler, GlobSuccessHandler};

#[derive(Debug, Clone, Serialize)]
#[napi(object)]
pub struct CheckDangerJsxPropsResponse {
  pub file_path: String,
  pub items: Vec<CheckDangerJsxPropsResponseItem>,
  pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[napi(object)]
pub struct CheckDangerJsxPropsResponseItem {
  pub name: String,
  pub ast_node: beans::AstNode,
}

impl Display for CheckDangerJsxPropsResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
  }
}

pub fn check_danger_jsx_props(
  danger_jsx_props: Vec<String>,
  args: utils::GlobArgs,
) -> anyhow::Result<Vec<CheckDangerJsxPropsResponse>> {
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
          if let AstKind::JSXAttributeItem(jsx_attribute_item) = node.kind() {
            if let JSXAttributeItem::Attribute(jsx_attribute) =
              jsx_attribute_item
            {
              if danger_jsx_props
                .contains(&jsx_attribute.name.get_identifier().name.to_string())
              {
                return Some(CheckDangerJsxPropsResponseItem {
                  name: jsx_attribute.name.get_identifier().name.to_string(),
                  ast_node: beans::AstNode::with_source_and_span(
                    &semantic.source_text(),
                    &jsx_attribute.span,
                  ),
                });
              }
            }
          }
          None
        })
        .collect::<Vec<_>>();
      Some(CheckDangerJsxPropsResponse {
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
      return Some(CheckDangerJsxPropsResponse {
        file_path: relative_path,
        items: vec![],
        errors: vec![error],
      });
    },
    &args,
  )?;

  Ok(responses)
}
