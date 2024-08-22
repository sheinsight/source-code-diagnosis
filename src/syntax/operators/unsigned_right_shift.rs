use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./unsigned_right_shift.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_binary_expression.push(walk_binary_expression);
  },

  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::ShiftRightZeroFill)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_unsigned_right_shift",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
console.log(8 >>> 2);
"#,
    1,
  }

  // use crate::syntax::semantic_tester::SemanticTester;

  // use super::*;

  // fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
  //   usage
  //     .iter()
  //     .filter(|item| item.name == "operators_unsigned_right_shift")
  //     .count()
  // }

  // #[test]
  // fn should_ok_when_async_generator_function_declaration() {
  //   let mut tester =
  //     SemanticTester::from_visitor(UnsignedRightShiftVisitor::default());
  //   let usage = tester.analyze("console.log(8 >>> 2);");

  //   let count = get_async_function_count(&usage);

  //   assert_eq!(usage.len(), 1);

  //   assert_eq!(count, 1);
  // }
}
