use oxc_ast::{ast::ImportAttributeKey, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ImportImportAttributesTypeCssVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ImportImportAttributesTypeCssVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./import_import_attributes_type_css.json"))
        .unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ImportImportAttributesTypeCssVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ImportImportAttributesTypeCssVisitor {
  fn visit_with_clause(&mut self, it: &oxc_ast::ast::WithClause<'a>) {
    if it.attributes_keyword.name == "with" {
      for item in &it.with_entries {
        if let ImportAttributeKey::Identifier(key) = &item.key {
          if key.name == "type" && item.value.value == "css" {
            self
              .usage
              .push(CompatBox::new(it.span.clone(), self.compat.clone()));
          }
        }
      }
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
      .filter(|item| item.name == "import_import_attributes_type_css")
      .count()
  }

  #[test]
  fn should_ok_when_import_attributes_type_css() {
    let mut tester = SemanticTester::from_visitor(
      ImportImportAttributesTypeCssVisitor::default(),
    );
    let usage =
      tester.analyze("import json from './data.json' with { type: 'css' };");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_import_attributes_type_json() {
    let mut tester = SemanticTester::from_visitor(
      ImportImportAttributesTypeCssVisitor::default(),
    );
    let usage =
      tester.analyze("import json from './data.json' with { type: 'json' };");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 0);

    assert_eq!(count, 0);
  }
}
