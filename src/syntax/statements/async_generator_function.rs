use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct AsyncGeneratorFunctionVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for AsyncGeneratorFunctionVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./async_generator_function.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for AsyncGeneratorFunctionVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for AsyncGeneratorFunctionVisitor {
  fn visit_function(
    &mut self,
    it: &oxc_ast::ast::Function<'a>,
    flags: oxc_semantic::ScopeFlags,
  ) {
    if matches!(it.r#type, FunctionType::FunctionDeclaration) {
      if it.r#async && it.generator {
        self
          .usage
          .push(CompatBox::new(it.span.clone(), self.compat.clone()));
      }
    }

    walk::walk_function(self, it, flags);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "async_generator_function")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(AsyncGeneratorFunctionVisitor::default());
    let usage = tester.analyze("async function* foo() {}");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_fail_when_async_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(AsyncGeneratorFunctionVisitor::default());

    let usage = tester.analyze("async function foo() {}");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 0);

    assert_eq!(count, 0);
  }

  #[test]
  fn should_fail_when_generate_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(AsyncGeneratorFunctionVisitor::default());

    let usage = tester.analyze("function* foo() {}");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 0);

    assert_eq!(count, 0);
  }

  #[test]
  fn should_fail_when_async_generate_function_expression() {
    let mut tester =
      SemanticTester::from_visitor(AsyncGeneratorFunctionVisitor::default());

    let usage = tester.analyze("const a = function* () {}");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 0);

    assert_eq!(count, 0);
  }
}
