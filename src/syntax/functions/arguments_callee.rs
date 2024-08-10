use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ArgumentsCalleeVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ArgumentsCalleeVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./arguments_callee.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ArgumentsCalleeVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ArgumentsCalleeVisitor {
  fn visit_identifier_reference(
    &mut self,
    it: &oxc_ast::ast::IdentifierReference<'a>,
  ) {
    if it.name == "arguments" {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }

    walk::walk_identifier_reference(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "functions_arguments")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(ArgumentsCalleeVisitor::default());
    let usage = tester.analyze(
      "
function func1(a, b, c) {
  console.log(arguments[0]);
  // Expected output: 1

  console.log(arguments[1]);
  // Expected output: 2

  console.log(arguments[2]);
  // Expected output: 3
}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 3);

    assert_eq!(count, 3);
  }
}
