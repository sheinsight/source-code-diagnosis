use std::marker::PhantomData;

use oxc_ast::{visit::walk, AstKind, Visit};
use oxc_span::Span;
use serde::Deserialize;
use serde_json::from_str;

use crate::syntax::{
  compat::{Compat, CompatBox},
  operators::common_trait::CommonTrait,
};

pub struct HexadecimalNumericLiteralsVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for HexadecimalNumericLiteralsVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> HexadecimalNumericLiteralsVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let compat: Compat =
      from_str(include_str!("./hexadecimal_numeric_literals.json")).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      compat,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> Visit<'a> for HexadecimalNumericLiteralsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_numeric_literal(&mut self, it: &oxc_ast::ast::NumericLiteral<'a>) {
    if vec!["0x", "0X"].iter().any(|item| it.raw.starts_with(item)) {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: self.get_source_code(it.span).to_string(),
        compat: self.compat.clone(),
      });
    }
    walk::walk_numeric_literal(self, it);
  }
}

#[cfg(test)]
mod tests {

  use oxc_allocator::Allocator;

  use crate::syntax::operators::t::t_any;

  use super::*;

  #[test]
  fn should_test() {
    let source_code = r##"
0xFFFFFFFFFFFFFFFFF // 295147905179352830000
0x123456789ABCDEF   // 81985529216486900
0XA                 // 10    
"##;
    let allocator = Allocator::default();
    t_any(
      "hexadecimal_numeric_literals",
      source_code,
      &allocator,
      HexadecimalNumericLiteralsVisitor::new,
    );
  }
}
