use std::{fmt::Display, fs};

use napi_derive::napi;
use oxc_ast::{AstKind, ast::JSXAttributeItem};
use oxc_semantic::SemanticBuilder;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::Serialize;
use wax::Glob;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct JsArgs {
  pub cwd: String,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Args {
  pub cwd: String,
  pub pattern: String,
  pub ignore: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[napi(object)]
pub struct CheckDangerJsxPropsResponse {
  pub path: String,
  pub items: Vec<Item>,
  pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[napi(object)]
pub struct Item {
  pub name: String,
  pub ast_node: beans::AstNode,
}

impl Display for CheckDangerJsxPropsResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
  }
}

impl From<JsArgs> for Args {
  fn from(args: JsArgs) -> Self {
    let cwd = camino::Utf8PathBuf::from(&args.cwd).join("").to_string();
    let pattern = args.pattern.unwrap_or("**/*.{js,ts,jsx,tsx}".to_owned());
    let ignore = args.ignore.unwrap_or(vec![
      "**/node_modules/**".to_owned(),
      "**/*.d.ts".to_owned(),
    ]);
    Self {
      cwd,
      pattern,
      ignore,
    }
  }
}

pub fn check_danger_jsx_props(
  danger_jsx_props: Vec<String>,
  args: Args,
) -> anyhow::Result<Vec<CheckDangerJsxPropsResponse>> {
  let glob = Glob::new(&args.pattern)?;
  let files = glob
    .walk(&args.cwd)
    .not(args.ignore.iter().map(|s| s.as_str()))?;
  let response: Vec<CheckDangerJsxPropsResponse> = files
    .par_bridge()
    .filter_map(|item| {
      let entry = item.ok()?;
      let path = entry.path();
      let relative_path = pathdiff::diff_paths(path, &args.cwd)?;

      if !path.is_file() {
        return None;
      }

      if utils::is_ts_video(path) {
        return None;
      }

      let metadata = fs::metadata(path).ok()?;

      if metadata.len() > 1_000_000 {
        return Some(CheckDangerJsxPropsResponse {
          path: relative_path.to_string_lossy().to_string(),
          items: vec![],
          errors: vec!["file size is too large".to_owned()],
        });
      }
      match std::fs::read_to_string(path) {
        Err(err) => Some(CheckDangerJsxPropsResponse {
          path: relative_path.to_string_lossy().to_string(),
          items: vec![],
          errors: vec![err.to_string()],
        }),
        Ok(content) => {
          let allocator = oxc_allocator::Allocator::default();

          let source_type = match path.extension().and_then(|ext| ext.to_str())
          {
            Some("ts") => oxc_span::SourceType::ts(),
            Some("tsx") => oxc_span::SourceType::tsx(),
            Some("jsx") => oxc_span::SourceType::jsx(),
            Some("cjs") => oxc_span::SourceType::cjs(),
            // _ => oxc_span::SourceType::default(),
            _ => oxc_span::SourceType::jsx(),
          };

          let parser =
            oxc_parser::Parser::new(&allocator, &content, source_type);

          let parse_response = parser.parse();

          if parse_response.errors.len() > 0 {
            return Some(CheckDangerJsxPropsResponse {
              path: relative_path.to_string_lossy().to_string(),
              items: vec![],
              errors: parse_response
                .errors
                .iter()
                .map(|e| e.to_string())
                .collect(),
            });
          }

          let program = allocator.alloc(parse_response.program);

          let semantic = SemanticBuilder::new()
            .with_check_syntax_error(true)
            .build(program);

          if semantic.errors.len() > 0 {
            return Some(CheckDangerJsxPropsResponse {
              path: relative_path.to_string_lossy().to_string(),
              items: vec![],
              errors: semantic.errors.iter().map(|e| e.to_string()).collect(),
            });
          }

          let nodes = semantic.semantic.nodes();

          let mut items: Vec<Item> = vec![];

          for node in nodes.iter() {
            if let AstKind::JSXAttributeItem(jsx_attribute_item) = node.kind() {
              if let JSXAttributeItem::Attribute(jsx_attribute) =
                jsx_attribute_item
              {
                if danger_jsx_props.contains(
                  &jsx_attribute.name.get_identifier().name.to_string(),
                ) {
                  items.push(Item {
                    name: jsx_attribute.name.get_identifier().name.to_string(),
                    ast_node: beans::AstNode::with_source_and_span(
                      &content,
                      &jsx_attribute.span,
                    ),
                  });
                }
              }
            }
          }

          Some(CheckDangerJsxPropsResponse {
            path: relative_path.to_string_lossy().to_string(),
            items,
            errors: vec![],
          })
        }
      }
    })
    .collect();
  Ok(response)
}
