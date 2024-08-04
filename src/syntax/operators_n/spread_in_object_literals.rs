use std::marker::PhantomData;

use oxc_ast::{ast::ObjectPropertyKind, AstKind, Visit};
use oxc_span::Span;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

pub struct SpreadVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for SpreadVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> SpreadVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let compat: Compat =
      from_str(include_str!("./spread_in_object_literals.json")).unwrap();
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

impl<'a> Visit<'a> for SpreadVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_object_expression(
    &mut self,
    expr: &oxc_ast::ast::ObjectExpression<'a>,
  ) {
    for prop in expr.properties.iter() {
      if let ObjectPropertyKind::SpreadProperty(p) = prop {
        self.cache.push(CompatBox {
          start: p.span.start,
          end: p.span.end,
          code_seg: self.get_source_code(p.span).to_string(),
          compat: self.compat.clone(),
        });
      }
    }

    oxc_ast::visit::walk::walk_object_expression(self, expr);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_spread_in_object_literals() {
    let source_code = r##"
const obj1 = { foo: "bar", x: 42 };
const obj2 = { bar: "baz", y: 13 };

const mergedObj = { ...obj1, ...obj2 };
        "##;
    let allocator = Allocator::default();
    t_any(
      "spread_in_object_literals",
      source_code,
      &allocator,
      SpreadVisitor::new,
    );
  }
}
