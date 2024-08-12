use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ShorthandObjectLiteralsVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for ShorthandObjectLiteralsVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./shorthand_object_literals.json")).unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for ShorthandObjectLiteralsVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ShorthandObjectLiteralsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_object_property(&mut self, it: &oxc_ast::ast::ObjectProperty<'a>) {
    if it.shorthand {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }

    walk::walk_object_property(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "shorthand_object_literals")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(ShorthandObjectLiteralsVisitor::default());
    let usage = tester.analyze(
      r##"
const name = "Alice";
const age = 30;

const person = {
  name,
  age,
  sayHello() {
    console.log(`Hello, I'm ${this.name}`);
  },
  [`status_${Date.now()}`]: "active"
};    
"##,
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }
}
