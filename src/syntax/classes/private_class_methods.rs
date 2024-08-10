use oxc_ast::{
  ast::{FunctionType, PropertyKey},
  visit::walk,
  Visit,
};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct PrivateClassMethodsVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for PrivateClassMethodsVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./private_class_methods.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for PrivateClassMethodsVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for PrivateClassMethodsVisitor {
  fn visit_method_definition(
    &mut self,
    it: &oxc_ast::ast::MethodDefinition<'a>,
  ) {
    if matches!(it.key, PropertyKey::PrivateIdentifier(_)) {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }
    walk::walk_method_definition(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "classes_private_class_methods")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(PrivateClassMethodsVisitor::default());
    let usage = tester.analyze(
      "
class ClassWithPrivate {
  #privateField;
  #privateFieldWithInitializer = 42;

  #privateMethod() {
    // …
  }

  static #privateStaticField;
  static #privateStaticFieldWithInitializer = 42;

  static #privateStaticMethod() {
    // …
  }
}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }
}
