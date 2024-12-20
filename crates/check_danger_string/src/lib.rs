use anyhow::Result;
use beans::AstNode;
use napi_derive::napi;
use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use serde::Serialize;
use utils::{glob_by, source_type_from_path, GlobArgs};

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
  let responses = glob_by(
    |path| {
      let relative_path = pathdiff::diff_paths(path, &args.cwd)?;

      let relative_path_str =
        utils::win_path_to_unix(relative_path.display().to_string().as_str());

      let source_code = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
          return Some(CheckDangerResponse {
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
        return Some(CheckDangerResponse {
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
        file_path: relative_path_str,
        items: responses,
        errors: vec![],
      })
    },
    &args,
  )?;

  Ok(responses)
}
