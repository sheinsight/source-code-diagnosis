use oxc_ast::{ast::VariableDeclarationKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct VarVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for VarVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./var.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for VarVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for VarVisitor {
  fn visit_variable_declaration(
    &mut self,
    it: &oxc_ast::ast::VariableDeclaration<'a>,
  ) {
    if matches!(it.kind, VariableDeclarationKind::Var) {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "var").count()
  }

  #[test]
  fn should_ok_when_use_var_declaration() {
    let mut tester = SemanticTester::from_visitor(VarVisitor::default());
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

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }

  #[test]
  fn should_fail_when_use_let_declaration() {
    let mut tester = SemanticTester::from_visitor(VarVisitor::default());
    let usage = tester.analyze(
      "
let x = 1;

if (x === 1) {
  let x = 2;

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
