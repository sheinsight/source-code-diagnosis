use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;
use oxc_syntax::operator::AssignmentOperator;

use crate::syntax::{compat::CompatBox, operators::Operators};

use super::common_trait::CommonTrait;

pub struct ExponentiationAssignmentVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  operators: Operators,
}

impl CommonTrait for ExponentiationAssignmentVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> ExponentiationAssignmentVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let operators_str = include_str!("./browser_compat_data/operators.json");
    let operators: Operators = serde_json::from_str(operators_str).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      operators: operators,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> Visit<'a> for ExponentiationAssignmentVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_assignment_expression(
    &mut self,
    expr: &oxc_ast::ast::AssignmentExpression<'a>,
  ) {
    if expr.operator == AssignmentOperator::Exponential {
      self.cache.push(CompatBox {
        start: expr.span.start,
        end: expr.span.end,
        code_seg: self.get_source_code(expr.span).to_string(),
        compat: self.operators.exponentiation_assignment.clone(),
      });
    }
    oxc_ast::visit::walk::walk_assignment_expression(self, expr);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exist_exponentiation_assignment() {
    let source_code = r##"
let a = 2;
a **= 2;
"##;
    let allocator = Allocator::default();
    t_any(
      "exponentiation_assignment",
      source_code,
      &allocator,
      ExponentiationAssignmentVisitor::new,
    );
  }
}
