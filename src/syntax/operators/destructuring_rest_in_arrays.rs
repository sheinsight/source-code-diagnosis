use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct DestructuringRestInArraysVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for DestructuringRestInArraysVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./destructuring_rest_in_arrays.json")).unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for DestructuringRestInArraysVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for DestructuringRestInArraysVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_array_pattern(&mut self, it: &oxc_ast::ast::ArrayPattern<'a>) {
    if it.rest.is_some() {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }
    walk::walk_array_pattern(self, it);
  }

  fn visit_array_assignment_target(
    &mut self,
    it: &oxc_ast::ast::ArrayAssignmentTarget<'a>,
  ) {
    if it.rest.is_some() {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()))
    }

    walk::walk_array_assignment_target(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "rest_in_arrays")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(DestructuringRestInArraysVisitor::default());
    let usage = tester.analyze(
      "
const [a, ...b] = array;    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_array_only_destructuring() {
    let mut tester =
      SemanticTester::from_visitor(DestructuringRestInArraysVisitor::default());
    let usage = tester.analyze(
      "
const [...b] = array;    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
