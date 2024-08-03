use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;
use oxc_syntax::{operator::AssignmentOperator, xml_entities};

use crate::syntax::{compat::CompatBox, operators::Operators};

pub struct ExponentiationAssignmentVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  operators: Operators,
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
  use oxc_allocator::Allocator;
  use oxc_parser::Parser;
  use oxc_span::SourceType;

  use super::*;

  fn t<F>(source_code: &str, assert_fn: F)
  where
    F: Fn(Vec<CompatBox>),
  {
    let mut visitor = ExponentiationAssignmentVisitor::new(&source_code);
    let allocator = Allocator::default();
    let source_type = SourceType::default();
    let ret = Parser::new(&allocator, source_code, source_type).parse();
    visitor.visit_program(&ret.program);
    assert_fn(visitor.cache);
  }

  #[test]
  fn should_exist_exponentiation_assignment() {
    t(
      r##"
let a = 2;
a **= 2;
"##,
      |res| {
        assert!(res
          .iter()
          .any(|compat| compat.compat.name == "exponentiation_assignment"));
      },
    );
  }
}
