use oxc_ast::{visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct LabelVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for LabelVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./label.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for LabelVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for LabelVisitor {
  fn visit_labeled_statement(
    &mut self,
    it: &oxc_ast::ast::LabeledStatement<'a>,
  ) {
    self
      .usage
      .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    walk::walk_labeled_statement(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "label").count()
  }

  #[test]
  fn should_ok_when_label_statements() {
    let mut tester = SemanticTester::from_visitor(LabelVisitor::default());
    let usage = tester.analyze(
      "
let str = '';

loop1: for (let i = 0; i < 5; i++) {
  if (i === 1) {
    continue loop1;
  }
  str = str + i;
}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
