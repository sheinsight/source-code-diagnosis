use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct DoWhileVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for DoWhileVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./do_while.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for DoWhileVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for DoWhileVisitor {
  fn visit_do_while_statement(
    &mut self,
    it: &oxc_ast::ast::DoWhileStatement<'a>,
  ) {
    self
      .usage
      .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    walk::walk_do_while_statement(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "do_while").count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(DoWhileVisitor::default());
    let usage = tester.analyze(
      "
let result = '';
let i = 0;
do {
  i = i + 1;
  result = result + i;
} while (i < 5);    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
