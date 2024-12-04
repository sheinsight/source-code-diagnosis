use anyhow::Result;
use napi_derive::napi;
use rayon::prelude::*;
use std::path::Path;
use wax::Glob;

use crate::is_ts_video;

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

pub fn glob_by<'a, F, T>(handler_fn: F, args: &GlobArgs) -> Result<Vec<T>>
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
