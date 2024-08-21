use std::sync::OnceLock;

use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::{CommonTrait, Context},
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

// pub struct HexadecimalNumericLiteralsVisitor<'a> {
//   usage: Vec<CompatBox>,
//   parent_stack: Vec<AstKind<'a>>,
//   compat: Compat,
// }

// impl<'a> Default for HexadecimalNumericLiteralsVisitor<'a> {
//   fn default() -> Self {
//     let usage: Vec<CompatBox> = Vec::new();
//     let compat: Compat =
//       from_str(include_str!("./hexadecimal_numeric_literals.json")).unwrap();
//     Self {
//       usage,
//       compat,
//       parent_stack: Vec::new(),
//     }
//   }
// }

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_numeric_literal(ctx: &mut Context, it: &oxc_ast::ast::NumericLiteral) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./hexadecimal_numeric_literals.json")).unwrap()
  });
  if vec!["0x", "0X"].iter().any(|item| it.raw.starts_with(item)) {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_hexadecimal_numeric_literals(v: &mut SyntaxVisitor) {
  v.walk_numeric_literal.push(walk_numeric_literal);
}

// impl<'a> CommonTrait for HexadecimalNumericLiteralsVisitor<'a> {
//   fn get_usage(&self) -> Vec<CompatBox> {
//     self.usage.clone()
//   }
// }

// impl<'a> Visit<'a> for HexadecimalNumericLiteralsVisitor<'a> {
//   fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
//     self.parent_stack.push(kind);
//   }

//   fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
//     self.parent_stack.pop();
//   }

//   fn visit_numeric_literal(&mut self, it: &oxc_ast::ast::NumericLiteral<'a>) {
//     if vec!["0x", "0X"].iter().any(|item| it.raw.starts_with(item)) {
//       self
//         .usage
//         .push(CompatBox::new(it.span.clone(), self.compat.clone()));
//     }
//     walk::walk_numeric_literal(self, it);
//   }
// }

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::hexadecimal_numeric_literals::setup_hexadecimal_numeric_literals,
  };

  assert_ok_count! {
    "hexadecimal_numeric_literals",
    setup_hexadecimal_numeric_literals,

    should_ok_when_use_hexadecimal_numeric_literals,
    r#"
      0xFFFFFFFFFFFFFFFFF; // 295147905179352830000
      0x123456789ABCDEF;   // 81985529216486900
      0XA;                 // 10
    "#,
    3,
  }

  //   use crate::syntax::semantic_tester::SemanticTester;

  //   use super::*;

  //   fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
  //     usage
  //       .iter()
  //       .filter(|item| item.name == "hexadecimal_numeric_literals")
  //       .count()
  //   }

  //   #[test]
  //   fn should_ok_when_async_generator_function_declaration() {
  //     let mut tester = SemanticTester::from_visitor(
  //       HexadecimalNumericLiteralsVisitor::default(),
  //     );
  //     let usage = tester.analyze(
  //       "
  // 0xFFFFFFFFFFFFFFFFF // 295147905179352830000
  // 0x123456789ABCDEF   // 81985529216486900
  // 0XA                 // 10
  // ",
  //     );

  //     let count = get_async_function_count(&usage);

  //     assert_eq!(usage.len(), 3);

  //     assert_eq!(count, 3);
  //   }
}
