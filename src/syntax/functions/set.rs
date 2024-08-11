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

pub struct SetVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for SetVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./set.json")).unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for SetVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for SetVisitor<'a> {
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
      let is_set = match parent {
        AstKind::ObjectProperty(parent) => PropertyKind::Set == parent.kind,
        AstKind::MethodDefinition(parent) => {
          MethodDefinitionKind::Set == parent.kind
        }
        _ => false,
      };
      if is_set {
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
    usage.iter().filter(|item| item.name == "set").count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(SetVisitor::default());
    let usage = tester.analyze(
      "
 const language = {
  set current(name) {
    this.log.push(name);
  },
  log: [],
};    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
