use oxc_ast::{ast::VariableDeclarationKind, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ConstVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ConstVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./const.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ConstVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ConstVisitor {
  fn visit_variable_declaration(
    &mut self,
    it: &oxc_ast::ast::VariableDeclaration<'a>,
  ) {
    if matches!(it.kind, VariableDeclarationKind::Const) {
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
    usage.iter().filter(|item| item.name == "const").count()
  }

  #[test]
  fn should_ok_when_const_declaration() {
    let mut tester = SemanticTester::from_visitor(ConstVisitor::default());
    let usage = tester.analyze("const number = 42;");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_two_const_declaration_nested() {
    let mut tester = SemanticTester::from_visitor(ConstVisitor::default());
    let usage = tester.analyze(
      "
const number = 42;
if (true) {
  const number = 42;
}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }
}
