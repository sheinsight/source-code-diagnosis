use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ImportVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ImportVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./import.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ImportVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ImportVisitor {
  fn visit_import_declaration(
    &mut self,
    it: &oxc_ast::ast::ImportDeclaration<'a>,
  ) {
    self
      .usage
      .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    walk::walk_import_declaration(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "import").count()
  }

  #[test]
  fn should_ok_when_import_default_specifier() {
    let mut tester = SemanticTester::from_visitor(ImportVisitor::default());
    let usage = tester.analyze("import defaultExport from 'module-name';");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_import_namespace_specifier() {
    let mut tester = SemanticTester::from_visitor(ImportVisitor::default());
    let usage = tester.analyze("import * as name from 'module-name';");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_import_specifier() {
    let mut tester = SemanticTester::from_visitor(ImportVisitor::default());
    let usage = tester.analyze("import { export1 } from 'module-name';");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_import_alias_specifier() {
    let mut tester = SemanticTester::from_visitor(ImportVisitor::default());
    let usage =
      tester.analyze("import { export1 as alias1 } from 'module-name';");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_import_declaration() {
    let mut tester = SemanticTester::from_visitor(ImportVisitor::default());
    let usage = tester.analyze("import 'module-name';");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
