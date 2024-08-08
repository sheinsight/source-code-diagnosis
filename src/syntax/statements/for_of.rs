use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ForOfVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ForOfVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./for_of.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ForOfVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ForOfVisitor {
  fn visit_for_of_statement(&mut self, it: &oxc_ast::ast::ForOfStatement<'a>) {
    if !it.r#await {
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
    usage.iter().filter(|item| item.name == "for_of").count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(ForOfVisitor::default());
    let usage = tester.analyze(
      "
const array1 = ['a', 'b', 'c'];
for (const element of array1) {
  console.log(element);
}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
