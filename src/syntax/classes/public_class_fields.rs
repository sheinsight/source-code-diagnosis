use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct PublicClassFieldsVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for PublicClassFieldsVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./public_class_fields.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for PublicClassFieldsVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for PublicClassFieldsVisitor {
  fn visit_property_definition(
    &mut self,
    it: &oxc_ast::ast::PropertyDefinition<'a>,
  ) {
    if !matches!(it.key, oxc_ast::ast::PropertyKey::PrivateIdentifier(_)) {
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
      .filter(|item| item.name == "classes_public_class_fields")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(PublicClassFieldsVisitor::default());
    let usage = tester.analyze(
      "
class MyClass {
  publicField = 'This is a public field';
  static staticPublicField = 'This is a static public field';

  constructor() {
    console.log(this.publicField);
  }
}     
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }
}
