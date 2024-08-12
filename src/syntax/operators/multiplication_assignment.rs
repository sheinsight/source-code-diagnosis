use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use oxc_syntax::operator::AssignmentOperator;
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct MultiplicationAssignmentVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for MultiplicationAssignmentVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./multiplication_assignment.json")).unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for MultiplicationAssignmentVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for MultiplicationAssignmentVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_assignment_expression(
    &mut self,
    it: &oxc_ast::ast::AssignmentExpression<'a>,
  ) {
    if matches!(it.operator, AssignmentOperator::Multiplication) {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }

    walk::walk_assignment_expression(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "operators_multiplication_assignment")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(MultiplicationAssignmentVisitor::default());
    let usage = tester.analyze(
      "
let a = 2;
a *= 3;
console.log(a);    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
