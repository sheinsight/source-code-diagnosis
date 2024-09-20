use anyhow::Result;
use log::debug;
use napi_derive::napi;
use rayon::prelude::*;
use std::{env::current_dir, path::PathBuf};
use wax::Glob;

#[derive(Debug, Clone)]
#[napi[object]]
pub struct GlobOptions {
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub cwd: Option<String>,
}

pub fn glob<'a, F>(handler_fn: F, options: Option<GlobOptions>) -> Result<()>
where
  F: Fn(PathBuf) + Send + Sync + 'a,
{
  let pattern = match &options {
    Some(GlobOptions {
      pattern: Some(pattern),
      ..
    }) => pattern,
    _ => "**/*.{js,ts,jsx,tsx}",
  };

  debug!("pattern: {}", pattern);

  let ignore_patterns = match &options {
    Some(GlobOptions {
      ignore: Some(ignore),
      ..
    }) => ignore.into_iter().map(|x| x.as_str()).collect(),
    _ => vec!["**/node_modules/**", "**/*.d.ts"],
  };

  debug!("ignore_patterns: {:?}", ignore_patterns);

  let cwd = match &options {
    Some(GlobOptions { cwd: Some(cwd), .. }) => cwd.clone(),
    _ => current_dir()?.display().to_string(),
  };

  debug!("cwd: {}", cwd);

  let glob = Glob::new(pattern)?;

  let entries: Vec<_> = glob.walk(&cwd).not(ignore_patterns)?.collect();

  if log::log_enabled!(log::Level::Debug) {
    entries.iter().try_for_each(|item| -> Result<()> {
      if let Ok(entry) = item {
        debug!("entry: {}", entry.path().display().to_string());
      }
      Ok(())
    })?;
  }

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
