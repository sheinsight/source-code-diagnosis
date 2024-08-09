use oxc_ast::{
  ast::{Declaration, FunctionType},
  visit::walk,
  Visit,
};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct LetVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for LetVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./let.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for LetVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for LetVisitor {
  fn visit_variable_declaration(
    &mut self,
    it: &oxc_ast::ast::VariableDeclaration<'a>,
  ) {
    if matches!(it.kind, oxc_ast::ast::VariableDeclarationKind::Let) {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }

    walk::walk_variable_declaration(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "let").count()
  }

  #[test]
  fn should_ok_when_let_declaration() {
    let mut tester = SemanticTester::from_visitor(LetVisitor::default());
    let usage = tester.analyze(
      "
let x = 1;

if (x === 1) {
  let x = 2;

  console.log(x);
  // Expected output: 2
}

console.log(x);
// Expected output: 1
    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }

  #[test]
  fn should_fail_when_var_declaration() {
    let mut tester = SemanticTester::from_visitor(LetVisitor::default());
    let usage = tester.analyze(
      "
var x = 1;

if (x === 1) {
  var x = 2;

  console.log(x);
}

console.log(x);
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 0);

    assert_eq!(count, 0);
  }
}
