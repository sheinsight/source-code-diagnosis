use oxc_ast::{visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ForAwaitOfVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ForAwaitOfVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./for_await_of.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ForAwaitOfVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ForAwaitOfVisitor {
  fn visit_for_of_statement(&mut self, it: &oxc_ast::ast::ForOfStatement<'a>) {
    if it.r#await {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
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
      .filter(|item| item.name == "for_await_of")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(ForAwaitOfVisitor::default());
    let usage = tester.analyze(
      "
async function* foo() {
  yield 1;
  yield 2;
}
(async function () {
  for await (const num of foo()) {
    console.log(num);
    // Expected output: 1

    break; // Closes iterator, triggers return
  }
})();
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
