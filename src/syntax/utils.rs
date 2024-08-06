use oxc_allocator::Allocator;
use oxc_ast::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;

use super::{common::CommonTrait, compat::CompatBox};

pub fn t<'a, V>(
  source_code: &'a str,
  allocator: &'a Allocator,
  create: fn() -> V,
) -> Vec<CompatBox>
where
  V: Visit<'a> + CommonTrait,
{
  let mut visitor = create();
  let source_type = SourceType::default();
  let parser = Parser::new(&allocator, source_code, source_type);
  let ret = parser.parse();
  visitor.visit_program(&ret.program);
  let cache = visitor.get_cache();
  cache
}
