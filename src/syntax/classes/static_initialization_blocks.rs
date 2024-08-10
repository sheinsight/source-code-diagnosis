use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct StaticInitializationBlocksVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for StaticInitializationBlocksVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./static_initialization_blocks.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for StaticInitializationBlocksVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for StaticInitializationBlocksVisitor {
  fn visit_static_block(&mut self, it: &oxc_ast::ast::StaticBlock<'a>) {
    self
      .usage
      .push(CompatBox::new(it.span.clone(), self.compat.clone()));

    walk::walk_static_block(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "classes_static_initialization_blocks")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(
      StaticInitializationBlocksVisitor::default(),
    );
    let usage = tester.analyze(
      "
class MyClass {
  static staticField1;
  static staticField2;

  static {
    this.staticField1 = 'Initialized in static block';
    console.log('Static initialization block called');
  }
 
}     
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
