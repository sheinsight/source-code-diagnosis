use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use oxc_syntax::operator::AssignmentOperator;
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct AssignmentVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for AssignmentVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./assignment.json")).unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for AssignmentVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for AssignmentVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_variable_declarator(
    &mut self,
    it: &oxc_ast::ast::VariableDeclarator<'a>,
  ) {
    if it.init.is_some() {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }

    walk::walk_variable_declarator(self, it);
  }

  fn visit_assignment_expression(
    &mut self,
    it: &oxc_ast::ast::AssignmentExpression<'a>,
  ) {
    if matches!(it.operator, AssignmentOperator::Assign) {
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
      .filter(|item| item.name == "operators_assignment")
      .count()
  }

  #[test]
  fn should_ok_when_basic_assignment() {
    let mut tester = SemanticTester::from_visitor(AssignmentVisitor::default());
    let usage = tester.analyze(
      "
let x = 5;    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_continuous_assignment() {
    let mut tester = SemanticTester::from_visitor(AssignmentVisitor::default());
    let usage = tester.analyze(
      "
let a, b, c;
a = b = c = 2;
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 3);

    assert_eq!(count, 3);
  }

  #[test]
  fn should_ok_when_deconstruct_assignment() {
    let mut tester = SemanticTester::from_visitor(AssignmentVisitor::default());
    let usage = tester.analyze(
      "
let [d, e] = [1, 2];
console.log(d, e);
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_object_deconstruct_assignment() {
    let mut tester = SemanticTester::from_visitor(AssignmentVisitor::default());
    let usage = tester.analyze(
      "
let {f, g} = {f: 3, g: 4};
console.log(f, g);
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
