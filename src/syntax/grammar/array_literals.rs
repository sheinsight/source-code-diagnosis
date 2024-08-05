use std::marker::PhantomData;

use oxc_ast::{visit::walk, AstKind, Visit};
use oxc_span::Span;
use serde_json::from_str;

use crate::syntax::{
  compat::{Compat, CompatBox},
  operators::common_trait::CommonTrait,
};

pub struct ArrayLiteralsVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for ArrayLiteralsVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> ArrayLiteralsVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let compat: Compat =
      from_str(include_str!("./array_literals.json")).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      compat: compat,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> Visit<'a> for ArrayLiteralsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_array_expression(&mut self, it: &oxc_ast::ast::ArrayExpression<'a>) {
    let code_seg = self.get_source_code(it.span).to_string();
    self.cache.push(CompatBox {
      start: it.span.start,
      end: it.span.end,
      code_seg: code_seg,
      compat: self.compat.clone(),
    });
    walk::walk_array_expression(self, it);
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
const fruits = ["Apple", "Banana"];    
"##;
    let allocator = Allocator::default();
    t_any(
      "array_literals",
      source_code,
      &allocator,
      ArrayLiteralsVisitor::new,
    );
  }
}
