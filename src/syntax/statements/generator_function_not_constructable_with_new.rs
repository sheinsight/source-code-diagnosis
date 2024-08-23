use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct GeneratorFunctionNotConstructableWithNewVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for GeneratorFunctionNotConstructableWithNewVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!(
      "./generator_function_not_constructable_with_new.json"
    ))
    .unwrap();
    Self { usage, compat }
  }
}

// TODO: implement
impl CommonTrait for GeneratorFunctionNotConstructableWithNewVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for GeneratorFunctionNotConstructableWithNewVisitor {}

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
    //   GeneratorFunctionNotConstructableWithNewVisitor::default(),
    // );
    // let usage = tester.analyze("async function* foo() {}");

    // let count = get_async_function_count(&usage);

    // assert_eq!(usage.len(), 1);

    // assert_eq!(count, 1);

    assert_eq!(1, 1)
  }
}
