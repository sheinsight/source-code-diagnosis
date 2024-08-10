use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct StaticVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for StaticVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./static.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for StaticVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for StaticVisitor {
  fn visit_method_definition(
    &mut self,
    it: &oxc_ast::ast::MethodDefinition<'a>,
  ) {
    if it.r#static {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }

    walk::walk_method_definition(self, it);
  }

  fn visit_property_definition(
    &mut self,
    it: &oxc_ast::ast::PropertyDefinition<'a>,
  ) {
    if it.r#static {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }

    walk::walk_property_definition(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "classes_static")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(StaticVisitor::default());
    let usage = tester.analyze(
      "
class MathOperations {
  static add(x, y) {
    return x + y;
  }

  static PI = 3.14159;
}
    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }
}
