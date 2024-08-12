use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct DestructuringComputedPropertyNamesVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for DestructuringComputedPropertyNamesVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./destructuring_computed_property_names.json"))
        .unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for DestructuringComputedPropertyNamesVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for DestructuringComputedPropertyNamesVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_object_pattern(&mut self, pat: &oxc_ast::ast::ObjectPattern<'a>) {
    for prop in pat.properties.iter() {
      if prop.computed {
        self
          .usage
          .push(CompatBox::new(pat.span.clone(), self.compat.clone()));
      }
    }

    walk::walk_object_pattern(self, pat);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "computed_property_names")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(
      DestructuringComputedPropertyNamesVisitor::default(),
    );
    let usage = tester.analyze(
      r#"
const key = "z";
const { [key]: a } = obj;    
"#,
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
