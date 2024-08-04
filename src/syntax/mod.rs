mod classes;
mod compat;
mod functions;
mod functions_n;
mod operators;
mod operators_n;
mod utils;
mod visitor;
use std::{
  fs::{self, read},
  path::{Path, PathBuf},
  sync::{Arc, Mutex},
};

use compat::{Compat, CompatBox};
use napi::{Error, Result};
use oxc_allocator::Allocator;
use oxc_ast::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;

use visitor::SyntaxRecordVisitor;

use crate::oxc_visitor_processor::{oxc_visit_process, Options};

#[napi]
pub fn check_browser_supported_by_file_path(
  target: String,
  file_path: String,
) -> Result<Vec<CompatBox>> {
  let f = Path::new(&file_path);

  let source_text = fs::read_to_string(f)?;

  let mut x = SyntaxRecordVisitor::new(source_text.as_str());

  // let source_text = String::from_utf8(source_text).unwrap();
  let source_type = SourceType::from_path(&f)
    .map_err(|e| Error::new(napi::Status::GenericFailure, e.0.to_string()))
    .unwrap();
  let allocator = Allocator::default();
  let ret = Parser::new(&allocator, &source_text, source_type).parse();

  x.visit_program(&ret.program);

  Ok(x.cache)
}

#[napi]
pub fn check_browser_supported(
  target: String,
  options: Option<Options>,
) -> Result<Vec<CompatBox>> {
  let used: Arc<Mutex<Vec<CompatBox>>> = Arc::new(Mutex::new(Vec::new()));
  let x = {
    let used = Arc::clone(&used);
    move |path: PathBuf| {
      let source_text = read(&path)
        .map_err(|err| {
          Error::new(
            napi::Status::GenericFailure,
            format!("Failed to read file: {}: {}", path.display(), err),
          )
        })
        .unwrap();
      let source_text = String::from_utf8(source_text).unwrap();
      let source_type = SourceType::from_path(&path)
        .map_err(|e| Error::new(napi::Status::GenericFailure, e.0.to_string()))
        .unwrap();
      let allocator = Allocator::default();
      let ret = Parser::new(&allocator, &source_text, source_type).parse();

      let mut x = SyntaxRecordVisitor::new(source_text.as_str());

      x.visit_program(&ret.program);

      // x.cache.sort_by_cached_key(|x| x.compat.support.chrome);

      let mut u = used.lock().unwrap();

      for item in x.cache.iter() {
        u.push(item.clone());
        // 片段
        // let seg = &source_text[item.start as usize..item.end as usize];
        // println!("syntax: {} ", item.compat.name);
        // println!(
        //   "chrome:{:<10} firefox:{:<10} safari:{:<10} edge:{:<10} opera:{:<10} deno:{:<10} node:{:<10}",
        //   item.compat.support.chrome,
        //   item.compat.support.firefox,
        //   item.compat.support.safari,
        //   item.compat.support.edge,
        //   item.compat.support.opera,
        //   item.compat.support.deno,
        //   item.compat.support.node
        // );
        // println!(
        //   "file: {} {}:{}",
        //   path.display().to_string(),
        //   item.start,
        //   item.end
        // );
        // println!("seg: {}", seg);

        // println!("-----------------------------------");
      }
    }
  };
  oxc_visit_process(x, options)?;

  let used = Arc::try_unwrap(used)
    .ok()
    .expect("Arc has more than one strong reference")
    .into_inner()
    .expect("Mutex cannot be locked");

  // println!("used: {:?}", used);

  // Ok(used)
  Ok(used)
}
