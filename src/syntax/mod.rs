mod classes;
mod common;
mod compat;
mod functions;
mod grammar;
mod operators;
mod statements;

mod semantic_tester;
mod visitor;
use std::{
  fs::{self, read},
  path::{Path, PathBuf},
  sync::{Arc, Mutex},
};

use classes::{
  constructor, extends, private_class_fields, private_class_fields_in,
  private_class_methods, public_class_fields, static_class_fields,
  static_initialization_blocks,
};
use compat::CompatBox;
use napi::{Error, Result};

use oxc_allocator::Allocator;
use oxc_ast::Visit as _;
use oxc_parser::Parser;
use oxc_span::SourceType;
use semantic_tester::SemanticTester;
use statements::{
  async_function::AsyncFunctionVisitor, class::ClassVisitor,
  r#const::ConstVisitor,
};
use visitor::SyntaxVisitor;

use crate::oxc_visitor_processor::{oxc_visit_process, Options};

// #[napi]
// pub fn check_browser_supported_by_file_path(
//   target: String,
//   file_path: String,
// ) -> Result<Vec<CompatBox>> {
//   let f = Path::new(&file_path);

//   let source_text = fs::read_to_string(f)?;

//   let mut x = SyntaxRecordVisitor::new(source_text.as_str());

//   // let source_text = String::from_utf8(source_text).unwrap();
//   let source_type = SourceType::from_path(&f)
//     .map_err(|e| Error::new(napi::Status::GenericFailure, e.0.to_string()))
//     .unwrap();
//   let allocator = Allocator::default();
//   let ret = Parser::new(&allocator, &source_text, source_type).parse();

//   x.visit_program(&ret.program);

//   // Ok(x.cache)
// }

#[napi]
pub fn check_browser_supported(
  target: String,
  options: Option<Options>,
) -> Result<Vec<CompatBox>> {
  let used: Arc<Mutex<Vec<CompatBox>>> = Arc::new(Mutex::new(Vec::new()));
  let handler = {
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

      let source_code = String::from_utf8(source_text).unwrap();
      let source_type = SourceType::from_path(&path)
        .map_err(|e| Error::new(napi::Status::GenericFailure, e.0.to_string()))
        .unwrap();
      let allocator = Allocator::default();
      let ret = Parser::new(&allocator, &source_code, source_type).parse();

      let mut v = SyntaxVisitor::new(source_code.as_str());

      v.walk_class_body.push(constructor::walk_class_body);

      v.walk_private_in_expression
        .push(private_class_fields_in::walk_private_in_expression);

      v.walk_class.push(extends::walk_class);

      v.walk_property_definition.extend([
        private_class_fields::walk_property_definition,
        public_class_fields::walk_property_definition,
        static_class_fields::walk_property_definition,
      ]);

      v.walk_method_definition
        .push(private_class_methods::walk_method_definition);

      v.walk_static_block
        .push(static_initialization_blocks::walk_static_block);

      // v.walk_static_block
      //   .extend([static_initialization_blocks::walk_static_block]);

      v.visit_program(&ret.program);

      let mut used = used.lock().unwrap();

      used.extend(v.context.usage);
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
