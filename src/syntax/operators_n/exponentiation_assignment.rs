use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;
use oxc_syntax::operator::AssignmentOperator;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

#[derive(Debug, serde::Deserialize)]
pub struct ExponentiationAssignmentCompatMetaData {
  pub exponentiation_assignment: Compat,
}

pub struct ExponentiationAssignmentVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  browser_compat_meta_data: ExponentiationAssignmentCompatMetaData,
}

impl CommonTrait for ExponentiationAssignmentVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> ExponentiationAssignmentVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let browser_compat_meta_data: ExponentiationAssignmentCompatMetaData =
      from_str(include_str!("./exponentiation_assignment.json")).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      browser_compat_meta_data: browser_compat_meta_data,
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
        compat: self
          .browser_compat_meta_data
          .exponentiation_assignment
          .clone(),
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
