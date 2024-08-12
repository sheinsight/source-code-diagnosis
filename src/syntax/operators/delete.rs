use oxc_ast::{visit::walk, AstKind, Visit};
use oxc_syntax::operator::UnaryOperator;
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct DeleteVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for DeleteVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./delete.json")).unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for DeleteVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for DeleteVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_unary_expression(&mut self, it: &oxc_ast::ast::UnaryExpression<'a>) {
    if matches!(it.operator, UnaryOperator::Delete) {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }
    walk::walk_unary_expression(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "delete").count()
  }

  #[test]
  fn should_ok_when_delete_object_key() {
    let mut tester = SemanticTester::from_visitor(DeleteVisitor::default());
    let usage = tester.analyze(
      "
const myObject = {x: 1, y: 2};
delete myObject.x;    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_delete_array_index() {
    let mut tester = SemanticTester::from_visitor(DeleteVisitor::default());
    let usage = tester.analyze(
      "
const arr = [1, 2, 3, 4];
delete arr[2];
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
