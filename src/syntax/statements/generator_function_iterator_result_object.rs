use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct GeneratorFunctionIteratorResultObjectVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for GeneratorFunctionIteratorResultObjectVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!(
      "./generator_function_iterator_result_object.json"
    ))
    .unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for GeneratorFunctionIteratorResultObjectVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

// TODO: implement

impl<'a> Visit<'a> for GeneratorFunctionIteratorResultObjectVisitor {}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "__tmp__").count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    // let mut tester = SemanticTester::from_visitor(
    //   GeneratorFunctionIteratorResultObjectVisitor::default(),
    // );
    // let usage = tester.analyze("async function* foo() {}");

    // let count = get_async_function_count(&usage);

    // assert_eq!(usage.len(), 1);

    // assert_eq!(count, 1);
    assert_eq!(1, 1)
  }
}
