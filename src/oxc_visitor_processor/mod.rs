use napi::{Error, Result};
use rayon::prelude::*;
use std::{
  env::current_dir,
  path::PathBuf,
  sync::{Arc, Mutex},
};
use threadpool::ThreadPool;
use wax::Glob;

#[derive(Debug, Clone)]
#[napi[object]]
pub struct Target {
  pub chrome: String,
  // pub firefox: Option<String>,
  // pub safari: Option<String>,
  // pub edge: Option<String>,
  // pub node: Option<String>,
  // pub deno: Option<String>,
}

#[derive(Debug, Clone)]
#[napi[object]]
pub struct Options {
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub cwd: Option<String>,
  pub concurrency: Option<i32>,
}

pub fn oxc_visit_process<F>(
  mut create_visit: F,
  options: Option<Options>,
) -> Result<()>
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

  let glob = match Glob::new(default_pattern) {
    Ok(glob) => glob,
    Err(e) => {
      return Err(Error::new(napi::Status::GenericFailure, e.to_string()))
    }
  };

  let entries: Vec<_> = glob
    .walk(&cwd)
    .not(ignore_patterns)
    .map_err(|e| Error::new(napi::Status::GenericFailure, e.to_string()))?
    .collect();

  // let create_visit = Arc::new(Mutex::new(create_visit));

  entries.par_iter().try_for_each(|item| -> Result<()> {
    if let Ok(entry) = item {
      if entry.path().is_file() {
        create_visit(entry.path().to_path_buf());
      }
    }
    Ok(())
  })?;

  // let create_visit = Arc::new(Mutex::new(create_visit));

  // // TODO use rayon 🤔 ？
  // let pool = ThreadPool::new(concurrency as usize);

  // for entry in entries {
  //   let entry = entry
  //     .map_err(|e| Error::new(napi::Status::GenericFailure, e.to_string()))?;
  //   let path = entry.path().to_path_buf();

  //   let create_visit = Arc::clone(&create_visit);

  //   if path.is_file() {
  //     pool.execute(move || {
  //       create_visit.lock().unwrap()(path.to_path_buf());
  //     });
  //   }
  // }

  // pool.join();

  Ok(())
}
