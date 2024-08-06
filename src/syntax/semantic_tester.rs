use std::marker::PhantomData;

use oxc_allocator::Allocator;
use oxc_ast::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;

use super::{common::CommonTrait, compat::CompatBox};

pub struct SemanticTester<'a, V>
where
  V: Visit<'a> + CommonTrait,
{
  allocator: Allocator,
  visitor: V,
  _marker: PhantomData<&'a ()>, // 使用 PhantomData 标记生命周期 'a
}

impl<'a, V> SemanticTester<'a, V>
where
  V: Visit<'a> + CommonTrait,
{
  fn new(visitor: V) -> Self {
    Self {
      allocator: Allocator::default(),
      visitor,
      _marker: PhantomData,
    }
  }

  pub fn with_visitor(visitor: V) -> Self {
    Self::new(visitor)
  }

  pub fn check(&'a mut self, source_code: &'a str) -> Vec<CompatBox> {
    let parser =
      Parser::new(&self.allocator, source_code, SourceType::default());
    let parse_return = parser.parse();
    let _ = &self.visitor.visit_program(&parse_return.program);
    self.visitor.get_usage()
  }
}
