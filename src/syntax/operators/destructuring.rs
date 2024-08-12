use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct DestructuringVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> DestructuringVisitor<'a> {
  fn is_destructuring(&self) -> bool {
    match self.parent_stack.last() {
      Some(AstKind::VariableDeclarator(_))
      | Some(AstKind::AssignmentTargetPattern(_)) => true,
      _ => false,
    }
  }
}

impl<'a> Default for DestructuringVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./destructuring.json")).unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for DestructuringVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for DestructuringVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_object_pattern(&mut self, pat: &oxc_ast::ast::ObjectPattern<'a>) {
    if self.is_destructuring() {
      self
        .usage
        .push(CompatBox::new(pat.span.clone(), self.compat.clone()));
    }

    walk::walk_object_pattern(self, pat);
  }

  fn visit_array_pattern(&mut self, it: &oxc_ast::ast::ArrayPattern<'a>) {
    if self.is_destructuring() {
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
    if self.is_destructuring() {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
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
      .filter(|item| item.name == "operators_destructuring")
      .count()
  }

  #[test]
  fn should_ok_when_destructuring_of_array() {
    let mut tester =
      SemanticTester::from_visitor(DestructuringVisitor::default());
    let usage = tester.analyze("const [a, b] = array;");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_destructuring_of_object() {
    let mut tester =
      SemanticTester::from_visitor(DestructuringVisitor::default());
    let usage = tester.analyze("const {a, b} = object;");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_destructuring_of_assignment() {
    let mut tester =
      SemanticTester::from_visitor(DestructuringVisitor::default());
    let usage = tester.analyze("const [a, b] = [1, 2];");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_destructuring_of_assignment_2() {
    let mut tester =
      SemanticTester::from_visitor(DestructuringVisitor::default());
    let usage = tester.analyze(
      "
let a, b;
[a, b] = array;    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_destructuring_of_computed_property_names() {
    let mut tester =
      SemanticTester::from_visitor(DestructuringVisitor::default());
    let usage = tester.analyze(
      r##"
const key = "z";
const { [key]: a } = obj;
"##,
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_destructuring_of_rest_in_arrays() {
    let mut tester =
      SemanticTester::from_visitor(DestructuringVisitor::default());
    let usage = tester.analyze(
      r##"
const [a, ...b] = array;
"##,
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_destructuring_of_rest_in_objects() {
    let mut tester =
      SemanticTester::from_visitor(DestructuringVisitor::default());
    let usage = tester.analyze(
      r##"
const {a, ...b} = object;
"##,
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_destructuring_of_for_of() {
    let mut tester =
      SemanticTester::from_visitor(DestructuringVisitor::default());
    let usage = tester.analyze(
      r##"
const people = [];
for (const {
  name: n,
  family: { father: f },
} of people) {
  console.log(`Name: ${n}, Father: ${f}`);
}
"##,
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
