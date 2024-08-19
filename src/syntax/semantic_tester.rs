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
  usage: Vec<CompatBox>,
  _marker: PhantomData<&'a ()>, // 使用 PhantomData 标记生命周期 'a
}

impl<'a, V> SemanticTester<'a, V>
where
  V: Visit<'a> + CommonTrait,
{
  pub fn from_visitor(visitor: V) -> Self {
    Self {
      allocator: Allocator::default(),
      visitor,
      usage: Vec::new(),
      _marker: PhantomData,
    }
  }

  pub fn analyze(&'a mut self, source_code: &'a str) -> Vec<CompatBox> {
    let parser =
      Parser::new(&self.allocator, source_code, SourceType::default());
    let parse_return = parser.parse();
    let _ = &self.visitor.visit_program(&parse_return.program);
    self.visitor.get_usage()
  }
}

#[macro_export]
macro_rules! assert_ok_count {
  ($name:expr, $setup:expr, $($(#[$attr:meta])* $test_name:ident, $source_code:expr, $expected_count:expr),* $(,)?) => {
    $(
      $(#[$attr])*
      #[test]
      fn $test_name() {
        let source_code = $source_code;
        let mut v = crate::syntax::visitor::SyntaxVisitor::new(source_code);
        $setup(&mut v);
        let allocator = oxc_allocator::Allocator::default();
        let source_type = oxc_span::SourceType::default();
        let parser = oxc_parser::Parser::new(&allocator, source_code, source_type);
        let parse_return = parser.parse();
        oxc_ast::Visit::visit_program(&mut v, &parse_return.program);
        let count = v
            .context
            .usage
            .iter()
            .filter(|item| item.name == $name)
            .count();
        assert_eq!(count, $expected_count);
      }
    )*
  };
}
