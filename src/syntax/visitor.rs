use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;

use super::compat::CompatBox;

#[derive(Debug)]
pub struct SyntaxRecordVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> SyntaxRecordVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> Visit<'a> for SyntaxRecordVisitor<'a> {}
