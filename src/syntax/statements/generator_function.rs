use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct GeneratorFunctionVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for GeneratorFunctionVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./generator_function.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for GeneratorFunctionVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for GeneratorFunctionVisitor {
  fn visit_function(
    &mut self,
    it: &oxc_ast::ast::Function<'a>,
    flags: oxc_semantic::ScopeFlags,
  ) {
    if matches!(it.r#type, FunctionType::FunctionDeclaration) {
      if !it.r#async && it.generator {
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
      .filter(|item| item.name == "generator_function")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(GeneratorFunctionVisitor::default());
    let usage = tester.analyze(
      "
function* generator(i) {
  yield i;
  yield i + 10;
}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_fail_when_async_generator_function_expression() {
    let mut tester =
      SemanticTester::from_visitor(GeneratorFunctionVisitor::default());
    let usage = tester.analyze(
      "
const generator = function* (i) {
  yield i;
  yield i + 10;
}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 0);

    assert_eq!(count, 0);
  }
}
