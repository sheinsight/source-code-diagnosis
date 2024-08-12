use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct DestructuringRestInObjectsVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for DestructuringRestInObjectsVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./destructuring_rest_in_objects.json")).unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for DestructuringRestInObjectsVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for DestructuringRestInObjectsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_object_pattern(&mut self, pat: &oxc_ast::ast::ObjectPattern<'a>) {
    if let Some(_) = pat.rest {
      self
        .usage
        .push(CompatBox::new(pat.span.clone(), self.compat.clone()));
    }

    walk::walk_object_pattern(self, pat);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "rest_in_objects")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(
      DestructuringRestInObjectsVisitor::default(),
    );
    let usage = tester.analyze("const {a, ...b} = object;");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_for_of() {
    let mut tester = SemanticTester::from_visitor(
      DestructuringRestInObjectsVisitor::default(),
    );
    let usage = tester.analyze(
      "
const people = [];
for (const {
  name: n,
  family: { father: f,...rest },

} of people) {
  console.log(`Name: ${n}, Father: ${f}`);
}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
