use std::marker::PhantomData;

use oxc_ast::{ast::Argument, AstKind, Visit};
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
      from_str(include_str!("./spread_in_function_calls.json")).unwrap();
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

impl<'a> Visit<'a> for SpreadVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
    for arg in &expr.arguments {
      if let Argument::SpreadElement(arg) = arg {
        self.cache.push(CompatBox {
          start: arg.span.start,
          end: arg.span.end,
          code_seg: self.get_source_code(arg.span).to_string(),
          compat: self.compat.clone(),
        });
      }
    }
    oxc_ast::visit::walk::walk_call_expression(self, expr);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_spread_in_function_calls() {
    let source_code = r##"
function myFunction(x, y, z) {}
const args = [0, 1, 2];
myFunction(...args); 
    "##;
    let allocator = Allocator::default();
    t_any(
      "spread_in_function_calls",
      source_code,
      &allocator,
      SpreadVisitor::new,
    );
  }
}
