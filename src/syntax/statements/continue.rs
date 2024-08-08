use oxc_ast::{visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ContinueVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ContinueVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./continue.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ContinueVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ContinueVisitor {
  fn visit_continue_statement(
    &mut self,
    it: &oxc_ast::ast::ContinueStatement<'a>,
  ) {
    self
      .usage
      .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    walk::walk_continue_statement(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "continue").count()
  }

  #[test]
  fn should_ok_when_for_statement() {
    let mut tester = SemanticTester::from_visitor(ContinueVisitor::default());
    let usage = tester.analyze(
      "
let text = '';

for (let i = 0; i < 10; i++) {
  if (i === 3) {
    continue;
  }
  text = text + i;
}

console.log(text);    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
