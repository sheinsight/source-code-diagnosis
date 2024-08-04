use oxc_allocator::Allocator;
use oxc_ast::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;

use super::common_trait::CommonTrait;

pub fn t<'a, V>(source_code: &'a str, allocator: &'a Allocator, visitor: &mut V)
where
  V: Visit<'a>,
{
  let source_type = SourceType::default();
  let parser = Parser::new(&allocator, source_code, source_type);
  let ret = parser.parse();
  visitor.visit_program(&ret.program);
}

pub fn t_any<'a, V>(
  name: &str,
  source_code: &'a str,
  allocator: &'a Allocator,
  create: fn(&'a str) -> V,
) where
  V: Visit<'a> + CommonTrait,
{
  let mut visitor = create(source_code);
  let source_type = SourceType::default();
  let parser = Parser::new(&allocator, source_code, source_type);
  let ret = parser.parse();
  visitor.visit_program(&ret.program);
  let cache = visitor.get_cache();
  assert!(cache.iter().any(|compat| compat.compat.name == name));
}

pub fn t_any_not<'a, V>(
  name: &str,
  source_code: &'a str,
  allocator: &'a Allocator,
  create: fn(&'a str) -> V,
) where
  V: Visit<'a> + CommonTrait,
{
  let mut visitor = create(source_code);
  let source_type = SourceType::default();
  let parser = Parser::new(&allocator, source_code, source_type);
  let ret = parser.parse();
  visitor.visit_program(&ret.program);
  let cache = visitor.get_cache();
  assert!(!cache.iter().any(|compat| compat.compat.name == name));
}
