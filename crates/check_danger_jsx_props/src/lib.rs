use std::fmt::Display;

use napi_derive::napi;
use oxc_ast::{AstKind, ast::JSXAttributeItem};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[napi(object)]
pub struct CheckDangerJsxPropsResponse {
  pub path: String,
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
  let responses = utils::glob_by(
    |path| {
      let relative_path = pathdiff::diff_paths(path, &args.cwd)?;

      let builder = utils::SemanticBuilder::with_file(path).unwrap();

      let semantic = builder.build().unwrap();

      let mut items: Vec<CheckDangerJsxPropsResponseItem> = vec![];

      for node in semantic.nodes().iter() {
        if let AstKind::JSXAttributeItem(jsx_attribute_item) = node.kind() {
          if let JSXAttributeItem::Attribute(jsx_attribute) = jsx_attribute_item
          {
            if danger_jsx_props
              .contains(&jsx_attribute.name.get_identifier().name.to_string())
            {
              items.push(CheckDangerJsxPropsResponseItem {
                name: jsx_attribute.name.get_identifier().name.to_string(),
                ast_node: beans::AstNode::with_source_and_span(
                  &semantic.source_text(),
                  &jsx_attribute.span,
                ),
              });
            }
          }
        }
      }

      Some(CheckDangerJsxPropsResponse {
        path: relative_path.display().to_string(),
        items,
        errors: vec![],
      })
    },
    &args,
  )?;

  Ok(responses)
}
