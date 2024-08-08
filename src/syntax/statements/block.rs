use oxc_ast::{visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct BlockVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for BlockVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./block.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for BlockVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for BlockVisitor {
  fn visit_block_statement(&mut self, it: &oxc_ast::ast::BlockStatement<'a>) {
    self
      .usage
      .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    walk::walk_block_statement(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "block").count()
  }

  #[test]
  fn should_ok_when_has_one_block() {
    let mut tester = SemanticTester::from_visitor(BlockVisitor::default());
    let usage = tester.analyze(
      "
var x = 1;
let y = 1;
if (true) {
  var x = 2;
  let y = 2;
}
console.log(x);
console.log(y);
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_nested_block() {
    let mut tester = SemanticTester::from_visitor(BlockVisitor::default());
    let usage = tester.analyze(
      "
var x = 1;
let y = 1;
if (true) {
  var x = 2;
  let y = 2;
  if (true) {
    console.log('two')
  }
}
console.log(x);
console.log(y);    
",
    );
    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }
}
