use oxc_ast::{visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ExportVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ExportVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./export.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ExportVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ExportVisitor {
  fn visit_export_named_declaration(
    &mut self,
    it: &oxc_ast::ast::ExportNamedDeclaration<'a>,
  ) {
    self
      .usage
      .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    walk::walk_export_named_declaration(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "export").count()
  }

  #[test]
  fn should_ok_when_exported_const_variable() {
    let mut tester = SemanticTester::from_visitor(ExportVisitor::default());
    let usage = tester.analyze("export const myVariable = 1;");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_exported_function_declaration() {
    let mut tester = SemanticTester::from_visitor(ExportVisitor::default());
    let usage = tester.analyze("export function myFunction() {}");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
