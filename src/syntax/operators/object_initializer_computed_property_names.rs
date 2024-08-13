use oxc_ast::{
  ast::{Expression, FunctionType, ObjectPropertyKind},
  visit::walk,
  AstKind, Visit,
};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ObjectInitializerComputedPropertyNamesVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for ObjectInitializerComputedPropertyNamesVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!(
      "./object_initializer_computed_property_names.json"
    ))
    .unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for ObjectInitializerComputedPropertyNamesVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ObjectInitializerComputedPropertyNamesVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_variable_declaration(
    &mut self,
    it: &oxc_ast::ast::VariableDeclaration<'a>,
  ) {
    for decl in &it.declarations {
      if let Some(init) = &decl.init {
        if let Expression::ObjectExpression(oe) = init {
          for p in &oe.properties {
            if let ObjectPropertyKind::ObjectProperty(op) = p {
              if op.computed {
                self
                  .usage
                  .push(CompatBox::new(it.span.clone(), self.compat.clone()));
              }
            }
          }
        }
      }
    }
    walk::walk_variable_declaration(self, it);
  }

  fn visit_assignment_expression(
    &mut self,
    it: &oxc_ast::ast::AssignmentExpression<'a>,
  ) {
    if let Expression::ObjectExpression(oe) = &it.right {
      for p in &oe.properties {
        if let ObjectPropertyKind::ObjectProperty(op) = p {
          if op.computed {
            self
              .usage
              .push(CompatBox::new(it.span.clone(), self.compat.clone()));
          }
        }
      }
    }
    walk::walk_assignment_expression(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| {
        item.name == "operators_object_initializer_computed_property_names"
      })
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(
      ObjectInitializerComputedPropertyNamesVisitor::default(),
    );
    let usage = tester.analyze(
      "
const propertyName = 'age';
const person = {
  [propertyName]: 30
};
console.log(person.age);
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration2() {
    let mut tester = SemanticTester::from_visitor(
      ObjectInitializerComputedPropertyNamesVisitor::default(),
    );
    let usage = tester.analyze(
      "
const propertyName = 'age';
let person;
person = {
  [propertyName]: 30
};
console.log(person.age);
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
