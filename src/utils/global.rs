use anyhow::Result;
use rayon::prelude::*;
use std::{env::current_dir, path::PathBuf};
use wax::Glob;

#[derive(Debug, Clone)]
#[napi[object]]
pub struct Options {
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub cwd: Option<String>,
  pub concurrency: Option<i32>,
}

pub fn glob<F>(handler_fn: F, options: Option<Options>) -> Result<()>
where
  F: Fn(PathBuf) + Send + Sync + 'static,
{
  let default_pattern: &str = "**/*.{js,ts,jsx,tsx}";

  let default_ignore_patterns: Vec<String> =
    vec!["**/node_modules/**".to_string(), "**/*.d.ts".to_string()];

  let ignore_patterns_vec: Vec<String> = options
    .as_ref()
    .and_then(|opts| opts.ignore.clone())
    .unwrap_or_else(|| {
      default_ignore_patterns
        .iter()
        .map(|s| s.to_string())
        .collect()
    });

  let ignore_patterns: Vec<&str> =
    ignore_patterns_vec.iter().map(String::as_str).collect();

  let dir = current_dir()?.display().to_string();

  let cwd = options
    .as_ref()
    .and_then(|opts| opts.cwd.clone())
    .unwrap_or(dir);

  let glob = Glob::new(default_pattern)?;

  let entries: Vec<_> = glob.walk(&cwd).not(ignore_patterns)?.collect();

  entries.par_iter().try_for_each(|item| -> Result<()> {
    if let Ok(entry) = item {
      if entry.path().is_file() {
        handler_fn(entry.path().to_path_buf());
      }
    }
    Ok(())
  })?;

  Ok(())
}
