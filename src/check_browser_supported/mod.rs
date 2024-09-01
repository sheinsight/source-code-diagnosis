// mod classes;
mod common;
mod compat;
// mod functions;
// mod grammar;
// mod macros;
// mod operators;
// mod statements;
mod classes_v2;
mod visitor;
use std::{
  fs::read,
  path::PathBuf,
  sync::{Arc, Mutex},
};

use compat::CompatBox;
use napi::{Error, Result};

use compat::AstNodeHelper as _;
use oxc_allocator::Allocator;
use oxc_ast::Visit as _;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::GetSpan;
use oxc_span::SourceType;
use visitor::SyntaxVisitor;

use crate::{
  check_browser_supported::{
    classes_v2::constructor::ClassesConstructor, compat::CompatHandler,
  },
  oxc_visitor_processor::{oxc_visit_process, Options},
};

#[napi]
pub fn check_browser_supported(
  target: String,
  options: Option<Options>,
) -> Result<Vec<CompatBox>> {
  let used: Arc<Mutex<Vec<CompatBox>>> = Arc::new(Mutex::new(Vec::new()));
  let handler = {
    let used = Arc::clone(&used);
    move |path: PathBuf| {
      let compat_handlers: Vec<Box<dyn CompatHandler>> =
        vec![Box::new(ClassesConstructor::default())];
      let source_text = read(&path)
        .map_err(|err| {
          Error::new(
            napi::Status::GenericFailure,
            format!("Failed to read file: {}: {}", path.display(), err),
          )
        })
        .unwrap();

      let source_code = String::from_utf8(source_text).unwrap();
      let source_type = SourceType::from_path(&path)
        .map_err(|e| Error::new(napi::Status::GenericFailure, e.0.to_string()))
        .unwrap();
      let allocator = Allocator::default();
      let ret = Parser::new(&allocator, &source_code, source_type).parse();
      let program = allocator.alloc(ret.program);
      let semantic = SemanticBuilder::new(&source_code, source_type)
        .build(program)
        .semantic;
      let nodes = semantic.nodes();

      for node in nodes.iter() {
        for compat_handler in compat_handlers.iter() {
          if compat_handler.handle(node, nodes) {
            let mut used = used.lock().unwrap();
            used.push(CompatBox::new(
              node.kind().span(),
              compat_handler.get_compat().clone(),
              path.to_str().unwrap().to_string(),
            ));
          }
        }
      }
    }
  };
  oxc_visit_process(handler, options)?;

  let used = Arc::try_unwrap(used)
    .ok()
    .expect("Arc has more than one strong reference")
    .into_inner()
    .expect("Mutex cannot be locked");

  Ok(used)
}
