use std::{fs::read_to_string, path::PathBuf};

use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_semantic::{AstNode, Semantic, SemanticBuilder as OxcSemanticBuilder};
use oxc_span::SourceType;

pub struct SemanticBuilder {
  pub source_code: String,
  pub source_type: SourceType,
  pub allocator: Allocator,
  pub path: PathBuf,
}

impl SemanticBuilder {
  pub fn new(path: PathBuf) -> Self {
    let allocator = Allocator::default();
    let source_code = read_to_string(&path).unwrap();
    let source_type = SourceType::from_path(&path).unwrap();
    Self {
      source_code,
      source_type,
      allocator,
      path,
    }
  }

  pub fn build_handler(&self) -> SemanticHandler {
    let ret =
      Parser::new(&self.allocator, &self.source_code, self.source_type).parse();
    let program = self.allocator.alloc(ret.program);
    let semantic = OxcSemanticBuilder::new(&self.source_code, self.source_type)
      .build(program)
      .semantic;
    SemanticHandler::new(semantic)
  }
}

pub struct SemanticHandler<'a> {
  pub semantic: Semantic<'a>,
}

impl<'a> SemanticHandler<'a> {
  pub fn new(semantic: Semantic<'a>) -> Self {
    Self { semantic }
  }

  pub fn each_node<F>(&self, f: F)
  where
    F: Fn(&AstNode),
  {
    for node in self.semantic.nodes().iter() {
      f(node);
    }
  }
}
