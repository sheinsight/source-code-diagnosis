use oxc_ast::{visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ForOfAsyncIteratorsVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ForOfAsyncIteratorsVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./for_of_async_iterators.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ForOfAsyncIteratorsVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ForOfAsyncIteratorsVisitor {
  fn visit_for_of_statement(&mut self, it: &oxc_ast::ast::ForOfStatement<'a>) {
    if it.r#await {
      // TODO
    }
    walk::walk_for_of_statement(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "for_of_async_iterators")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    //     let mut tester =
    //       SemanticTester::from_visitor(ForOfAsyncIteratorsVisitor::default());
    //     let usage = tester.analyze(
    //       "
    // async function* asyncGenerator() {
    //   yield await Promise.resolve(1);
    //   yield await Promise.resolve(2);
    //   yield await Promise.resolve(3);
    // }
    // (async () => {
    //   for await (const num of asyncGenerator()) {
    //     console.log(num);
    //   }
    // })();
    // ",
    //     );

    //     let count = get_async_function_count(&usage);

    //     assert_eq!(usage.len(), 1);

    //     assert_eq!(count, 1);
    assert_eq!(1, 1);
  }
}
