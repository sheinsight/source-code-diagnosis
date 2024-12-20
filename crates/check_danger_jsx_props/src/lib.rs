use std::fmt::Display;

use napi_derive::napi;
use oxc_allocator::Allocator;
use oxc_ast::{AstKind, ast::JSXAttributeItem};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use serde::Serialize;
use utils::source_type_from_path;

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
  let responses = utils::glob_by(
    |path| {
      let relative_path = pathdiff::diff_paths(path, &args.cwd)?;

      let relative_path_str =
        utils::win_path_to_unix(relative_path.display().to_string().as_str());

      let source_code = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
          return Some(CheckDangerJsxPropsResponse {
            file_path: relative_path_str,
            items: vec![],
            errors: vec![format!("文件读取错误: {}", err)],
          });
        }
      };

      let allocator = Allocator::default();
      let source_type = source_type_from_path(path);

      let parser = Parser::new(&allocator, &source_code, source_type);
      let parse = parser.parse();

      if !parse.errors.is_empty() {
        eprintln!("parse error: {:?}", parse.errors);
        return Some(CheckDangerJsxPropsResponse {
          file_path: relative_path_str,
          items: vec![],
          errors: vec![format!("解析错误: {:?}", parse.errors)],
        });
      }

      let program = allocator.alloc(parse.program);

      let semantic_return = SemanticBuilder::new()
        .with_check_syntax_error(false)
        .build(program);

      let semantic = semantic_return.semantic;

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
        file_path: relative_path_str,
        items: responses,
        errors: vec![],
      })
    },
    &args,
  )?;

  Ok(responses)
}
