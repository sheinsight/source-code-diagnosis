use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct MethodDefinitionsGeneratorMethodsNotConstructableVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default
  for MethodDefinitionsGeneratorMethodsNotConstructableVisitor<'a>
{
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!(
      "./method_definitions_generator_methods_not_constructable.json"
    ))
    .unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait
  for MethodDefinitionsGeneratorMethodsNotConstructableVisitor<'a>
{
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a>
  for MethodDefinitionsGeneratorMethodsNotConstructableVisitor<'a>
{
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
      let is_generator = match parent {
        AstKind::ObjectProperty(_) | AstKind::MethodDefinition(_) => {
          !it.r#async && it.generator
        }
        _ => false,
      };
      if is_generator {
        self
          .usage
          .push(CompatBox::new(it.span.clone(), self.compat.clone()));
      }
    }

    oxc_ast::visit::walk::walk_function(self, it, flags);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "generator_methods_not_constructable")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(
      MethodDefinitionsGeneratorMethodsNotConstructableVisitor::default(),
    );
    let usage = tester.analyze(
      "
const obj = {
  g: function* () {
    let index = 0;
    while (true) {
      yield index++;
    }
  },
};     
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
