use oxc_ast::{ast::ObjectPropertyKind, AstKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct SpreadSpreadInObjectLiteralsVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for SpreadSpreadInObjectLiteralsVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./spread_spread_in_object_literals.json"))
        .unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for SpreadSpreadInObjectLiteralsVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for SpreadSpreadInObjectLiteralsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_object_expression(
    &mut self,
    expr: &oxc_ast::ast::ObjectExpression<'a>,
  ) {
    for prop in expr.properties.iter() {
      if let ObjectPropertyKind::SpreadProperty(p) = prop {
        self
          .usage
          .push(CompatBox::new(p.span.clone(), self.compat.clone()));
      }
    }

    oxc_ast::visit::walk::walk_object_expression(self, expr);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "spread_in_object_literals")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(
      SpreadSpreadInObjectLiteralsVisitor::default(),
    );
    let usage = tester.analyze(
      r#"
const obj1 = { foo: "bar", x: 42 };
const obj2 = { bar: "baz", y: 13 };
const mergedObj = { ...obj1, ...obj2 };    
"#,
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }
}
