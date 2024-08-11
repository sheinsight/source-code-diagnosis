use oxc_ast::{
  ast::{FunctionType, MethodDefinitionKind, PropertyKind},
  visit::walk,
  AstKind, Visit,
};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct GetComputedPropertyNamesVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for GetComputedPropertyNamesVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./get_computed_property_names.json")).unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for GetComputedPropertyNamesVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for GetComputedPropertyNamesVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }
  fn visit_function(
    &mut self,
    it: &oxc_ast::ast::Function<'a>,
    flags: oxc_syntax::scope::ScopeFlags,
  ) {
    if let Some(parent) = self.parent_stack.last() {
      let is_get_in_computed = match parent {
        AstKind::ObjectProperty(parent) => {
          PropertyKind::Get == parent.kind && parent.computed
        }
        AstKind::MethodDefinition(parent) => {
          MethodDefinitionKind::Get == parent.kind && parent.computed
        }
        _ => false,
      };

      if is_get_in_computed {
        self
          .usage
          .push(CompatBox::new(it.span.clone(), self.compat.clone()));
      }
    }
    walk::walk_function(self, it, flags);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "get_computed_property_names")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(GetComputedPropertyNamesVisitor::default());
    let usage = tester.analyze(
      r##"
const expr = "foo";

const obj = {
  get [expr]() {
    return "bar";
  },
};    
"##,
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
