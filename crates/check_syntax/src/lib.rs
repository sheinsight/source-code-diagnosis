use std::fs;

use napi_derive::napi;
use rayon::prelude::*;
use utils::{is_ts_video, read_file_content};
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

#[derive(Debug, Clone)]
#[napi(object)]
pub struct CheckSyntaxResponse {
  pub path: String,
  pub errors: Vec<String>,
}

pub fn check_syntax(args: Args) -> anyhow::Result<Vec<CheckSyntaxResponse>> {
  let glob = Glob::new(&args.pattern)?;

  let files = glob
    .walk(&args.cwd)
    .not(args.ignore.iter().map(|s| s.as_str()))?;

  Ok(
    files
      .par_bridge()
      .filter_map(|item| {
        let entry = item.ok()?;
        let path = entry.path();
        let path_string = path.to_string_lossy().to_string();

        if !path.is_file() {
          return None;
        }

        if is_ts_video(path) {
          return None;
        }

        let metadata = fs::metadata(path).ok()?;

        if metadata.len() > 1_000_000 {
          return Some(CheckSyntaxResponse {
            path: path_string,
            errors: vec!["File is too large".to_owned()],
          });
        }

        match read_file_content(path) {
          Err(err) => Some(CheckSyntaxResponse {
            path: path_string,
            errors: vec![err.to_string()],
          }),
          Ok(content) => {
            let allocator = oxc_allocator::Allocator::default();

            let source_type =
              match path.extension().and_then(|ext| ext.to_str()) {
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

            (!parse_response.errors.is_empty()).then(|| CheckSyntaxResponse {
              path: pathdiff::diff_paths(&path, &args.cwd)
                .unwrap()
                .to_string_lossy()
                .to_string(),
              errors: parse_response
                .errors
                .iter()
                .map(|e| e.to_string())
                .collect(),
            })
          }
        }
      })
      .collect(),
  )
}
