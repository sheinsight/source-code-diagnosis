use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ArgumentsIteratorVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for ArgumentsIteratorVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./arguments_iterator.json")).unwrap();
    let parent_stack: Vec<AstKind<'_>> = Vec::new();
    Self {
      usage,
      compat,
      parent_stack,
    }
  }
}

impl<'a> CommonTrait for ArgumentsIteratorVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ArgumentsIteratorVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_identifier_reference(
    &mut self,
    it: &oxc_ast::ast::IdentifierReference<'a>,
  ) {
    if it.name == "arguments" {
      if let Some(p) = self.parent_stack.last() {
        if let AstKind::ForOfStatement(_) = p {
          self
            .usage
            .push(CompatBox::new(it.span.clone(), self.compat.clone()));
        }
      }
    }
    walk::walk_identifier_reference(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "functions_arguments_iterator")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(ArgumentsIteratorVisitor::default());
    let usage = tester.analyze(
      r##"
function f() {
  for (const letter of arguments) {
    console.log(letter);
  }
}
f("w", "y", "k", "o", "p");    
"##,
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
