use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ExtendsVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ExtendsVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./extends.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ExtendsVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ExtendsVisitor {
  fn visit_class(&mut self, it: &oxc_ast::ast::Class<'a>) {
    if it.super_class.is_some() {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }

    walk::walk_class(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "classes_extends")
      .count()
  }

  #[test]
  fn should_ok_when_class_declaration() {
    let mut tester = SemanticTester::from_visitor(ExtendsVisitor::default());
    let usage = tester.analyze(
      "
class DateFormatter extends Date {
  getFormattedDate() {
    const months = [
      'Jan',
      'Feb',
      'Mar',
      'Apr',
      'May',
      'Jun',
      'Jul',
      'Aug',
      'Sep',
      'Oct',
      'Nov',
      'Dec',
    ];
    return `${this.getDate()}-${months[this.getMonth()]}-${this.getFullYear()}`;
  }
}
     
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_class_expression() {
    let mut tester = SemanticTester::from_visitor(ExtendsVisitor::default());
    let usage = tester.analyze(
      "
const A = class extends Date {
  getFormattedDate() {
    const months = [
      'Jan',
      'Feb',
      'Mar',
      'Apr',
      'May',
      'Jun',
      'Jul',
      'Aug',
      'Sep',
      'Oct',
      'Nov',
      'Dec',
    ];
    return `${this.getDate()}-${months[this.getMonth()]}-${this.getFullYear()}`;
  }
}
     
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
