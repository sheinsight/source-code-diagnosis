use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ImportImportAttributesVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ImportImportAttributesVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./import_import_attributes.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ImportImportAttributesVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ImportImportAttributesVisitor {
  fn visit_with_clause(&mut self, it: &oxc_ast::ast::WithClause<'a>) {
    if it.attributes_keyword.name == "with" {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }
    walk::walk_with_clause(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "import_import_attributes")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(ImportImportAttributesVisitor::default());
    let usage =
      tester.analyze("import json from './data.json' with { type: 'json' };");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
