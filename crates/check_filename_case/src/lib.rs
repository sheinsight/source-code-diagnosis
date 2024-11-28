use std::collections::HashSet;

use napi_derive::napi;
use rayon::prelude::*;
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
      "node_modules/**".to_owned(),
    ]);
    Self {
      cwd,
      pattern,
      ignore,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[napi(object)]
pub struct CheckFilenameCaseResponse {
  pub path: String,
}

pub fn check_filename_case(
  args: Args,
) -> anyhow::Result<Vec<CheckFilenameCaseResponse>> {
  let glob = Glob::new(&args.pattern)?;

  let files = glob
    .walk(&args.cwd)
    .not(args.ignore.iter().map(|s| s.as_str()))?;

  let res: Vec<CheckFilenameCaseResponse> = files
    .par_bridge()
    .filter_map(|item| {
      let entry = item.ok()?;
      let path = entry.path();

      let path = pathdiff::diff_paths(path, &args.cwd)?;

      let str = path.display().to_string();

      let mut p = vec![];
      let path_str = str.split("/").collect::<Vec<&str>>();
      let len = path_str.len();
      for (index, part) in path_str.into_iter().enumerate() {
        p.push(part);
        if !part.to_ascii_lowercase().eq(part) {
          break;
        }
        if index == len - 1 {
          p.clear();
        }
      }

      if !p.is_empty() {
        return Some(CheckFilenameCaseResponse { path: p.join("/") });
      }

      return None;
    })
    .collect();

  let unique: Vec<_> = HashSet::<_>::from_iter(res).into_iter().collect();

  Ok(unique)
}
