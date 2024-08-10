use oxc_ast::{ast::ClassElement, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ConstructorVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ConstructorVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./constructor.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ConstructorVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ConstructorVisitor {
  fn visit_class_body(&mut self, it: &oxc_ast::ast::ClassBody<'a>) {
    it.body.iter().for_each(|item| {
      if let ClassElement::MethodDefinition(method_definition) = item {
        if let Some(name) = method_definition.key.name() {
          if name == "constructor" {
            self.usage.push(CompatBox::new(
              method_definition.span.clone(),
              self.compat.clone(),
            ));
          }
        }
      }
    });

    walk::walk_class_body(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "classes_constructor")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(ConstructorVisitor::default());
    let usage = tester.analyze(
      "
class Polygon {
  constructor() {
    this.name = 'Polygon';
  }
}

const poly1 = new Polygon();

console.log(poly1.name);     
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
