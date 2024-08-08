use oxc_ast::{visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct EmptyVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for EmptyVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./empty.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for EmptyVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for EmptyVisitor {
  fn visit_empty_statement(&mut self, it: &oxc_ast::ast::EmptyStatement) {
    self
      .usage
      .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    walk::walk_empty_statement(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "empty").count()
  }

  #[test]
  fn should_ok_when_switch_expression() {
    let mut tester = SemanticTester::from_visitor(EmptyVisitor::default());
    let usage = tester.analyze(
      "
switch (condition) {
  case 1:
    break;
  case 2:
  case 3:
    ; 
    break;
  default:
}
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_for_statement() {
    let mut tester = SemanticTester::from_visitor(EmptyVisitor::default());
    let usage = tester.analyze(
      "
for (let i = 0; i < array1.length; array1[i++] = 0)  ;
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
