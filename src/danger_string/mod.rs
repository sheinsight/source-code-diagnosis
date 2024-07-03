use std::{
  env::current_dir,
  fs::read,
  sync::{Arc, Mutex},
};

use danger_string_visitor::DangerStringVisitor;
use location::Location;
use napi::{Error, Result};
use oxc_allocator::Allocator;
use oxc_ast::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;
use threadpool::ThreadPool;
use wax::Glob;

mod danger_string_visitor;
mod location;

#[derive(Debug, Clone)]
#[napi[object]]
pub struct Options {
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub cwd: Option<String>,
  pub concurrency: Option<i32>,
}

#[napi]
pub fn get_usage_of_danger_strings(
  danger_strings: Vec<String>,
  options: Option<Options>,
) -> Result<Vec<Location>> {
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

  let ignore_patterns: Vec<&str> = ignore_patterns_vec.iter().map(String::as_str).collect();

  let dir = match current_dir() {
    Ok(dir) => dir.display().to_string(),
    Err(e) => {
      return Err(Error::new(napi::Status::GenericFailure, e.to_string()));
    }
  };

  let cwd = options
    .as_ref()
    .and_then(|opts| opts.cwd.clone())
    .unwrap_or(dir);

  let concurrency = options
    .as_ref()
    .and_then(|opts| opts.concurrency)
    .unwrap_or(4);

  let glob = match Glob::new(default_pattern) {
    Ok(glob) => glob,
    Err(e) => return Err(Error::new(napi::Status::GenericFailure, e.to_string())),
  };

  let entries = glob
    .walk(&cwd)
    .not(ignore_patterns)
    .map_err(|e| Error::new(napi::Status::GenericFailure, e.to_string()))?;

  let used = Arc::new(Mutex::new(Vec::new()));
  let danger_strings = Arc::new(danger_strings);

  let pool = ThreadPool::new(concurrency as usize);

  for entry in entries {
    let entry = entry.map_err(|e| Error::new(napi::Status::GenericFailure, e.to_string()))?;
    let path = entry.path().to_path_buf();
    let used = Arc::clone(&used);
    let danger_strings = Arc::clone(&danger_strings);

    pool.execute(move || {
      if path.is_file() {
        let source_text = read(&path)
          .map_err(|err| {
            Error::new(
              napi::Status::GenericFailure,
              format!("Failed to read file: {}: {}", path.display(), err),
            )
          })
          .unwrap();

        let source_text = String::from_utf8_lossy(&source_text);

        let allocator = Allocator::default();
        let source_type = SourceType::from_path(&path)
          .map_err(|e| Error::new(napi::Status::GenericFailure, e.0.to_string()))
          .unwrap();
        let ret = Parser::new(&allocator, &source_text, source_type).parse();
        let mut local_used = vec![];
        DangerStringVisitor {
          used: &mut local_used,
          file_path: &path.to_str().unwrap().to_string(),
          danger_strings: &danger_strings,
        }
        .visit_program(&ret.program);

        let mut used = used.lock().unwrap();
        used.extend(local_used);
      }
    });
  }

  pool.join();

  let used = Arc::try_unwrap(used).unwrap().into_inner().unwrap();

  Ok(used)
}
