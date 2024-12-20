use anyhow::Result;
use napi_derive::napi;
use oxc_allocator::Allocator;
use oxc_parser::{Parser, ParserReturn};
use oxc_semantic::{Semantic, SemanticBuilder, SemanticBuilderReturn};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use wax::Glob;

use crate::{is_ts_video, source_type_from_path, win_path_to_unix};

#[derive(Debug, Clone)]
pub struct GlobArgs {
  pub pattern: String,
  pub ignore: Vec<String>,
  pub cwd: String,
}

impl From<GlobJsArgs> for GlobArgs {
  fn from(args: GlobJsArgs) -> Self {
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

#[derive(Debug, Clone)]
#[napi[object]]
pub struct GlobJsArgs {
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub cwd: String,
}

pub fn glob_by_path<'a, F, T>(handler_fn: F, args: &GlobArgs) -> Result<Vec<T>>
where
  F: Fn(&Path) -> Option<T> + Send + Sync + 'a,
  T: Send + 'a,
{
  let glob = Glob::new(&args.pattern.as_str())?;

  let ignore_patterns = args
    .ignore
    .iter()
    .map(|s| s.as_str())
    .collect::<Vec<&str>>();

  let entries = glob.walk(args.cwd.as_str()).not(ignore_patterns)?;
  let responses = entries
    .par_bridge()
    .filter_map(|item| {
      let entry = item.ok()?;
      let path = entry.path();

      if !path.is_file() {
        return None;
      }

      if is_ts_video(path) {
        return None;
      }

      handler_fn(path)
    })
    .collect();

  Ok(responses)
}

pub struct GlobSuccessHandler<'a> {
  pub semantic: Semantic<'a>,
  pub parse: ParserReturn<'a>,
  pub path: PathBuf,
  pub absolute_path: String,
  pub relative_path: String,
}

pub struct GlobErrorHandler {
  pub path: PathBuf,
  pub absolute_path: String,
  pub relative_path: String,
  pub error: String,
}

pub fn glob_by_semantic<'a, F, E, T>(
  handler_fn: F,
  err_handler_fn: E,
  args: &GlobArgs,
) -> Result<Vec<T>>
where
  F: Fn(GlobSuccessHandler) -> Option<T> + Send + Sync + 'a,
  E: Fn(GlobErrorHandler) -> Option<T> + Send + Sync + 'a,
  T: Send + 'a,
{
  let glob = Glob::new(&args.pattern.as_str())?;

  let ignore_patterns = args
    .ignore
    .iter()
    .map(|s| s.as_str())
    .collect::<Vec<&str>>();

  let entries = glob.walk(args.cwd.as_str()).not(ignore_patterns)?;
  let responses = entries
    .par_bridge()
    .filter_map(|item| {
      let entry = item.ok()?;
      let path = entry.path();

      if !path.is_file() {
        return None;
      }

      if is_ts_video(path) {
        return None;
      }

      let relative_path = pathdiff::diff_paths(path, &args.cwd)?;
      let absolute_path = path.display().to_string();

      let relative_path_str =
        win_path_to_unix(relative_path.display().to_string().as_str());

      let source_code = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
          return err_handler_fn(GlobErrorHandler {
            path: path.to_path_buf(),
            absolute_path,
            relative_path: relative_path_str,
            error: format!("文件读取错误: {}", err),
          });
        }
      };

      let allocator = Allocator::default();

      let source_type = source_type_from_path(path);

      let parser = Parser::new(&allocator, &source_code, source_type);

      let parse = parser.parse();

      if !parse.errors.is_empty() {
        return err_handler_fn(GlobErrorHandler {
          path: path.to_path_buf(),
          absolute_path,
          relative_path: relative_path_str,
          error: format!("解析错误: {:?}", parse.errors),
        });
      }

      let program = allocator.alloc(&parse.program);

      let semantic_return = SemanticBuilder::new()
        .with_check_syntax_error(false)
        .build(program);

      if !semantic_return.errors.is_empty() {
        return err_handler_fn(GlobErrorHandler {
          path: path.to_path_buf(),
          absolute_path,
          relative_path: relative_path_str,
          error: format!("语义分析错误: {:?}", semantic_return.errors),
        });
      }

      let handler = GlobSuccessHandler {
        parse,
        path: path.to_path_buf(),
        semantic: semantic_return.semantic,
        absolute_path,
        relative_path: relative_path_str,
      };

      handler_fn(handler)
    })
    .collect();

  Ok(responses)
}
