use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./strict_equality.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_binary_expression.push(walk_binary_expression);
  },

  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::StrictEquality)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_strict_equality",
    setup,
    should_ok_when_strict_equality,
    r#"
      console.log(5 === 5);
    "#,
    1
  }

  // use crate::syntax::semantic_tester::SemanticTester;

  // use super::*;

  // fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
  //   usage
  //     .iter()
  //     .filter(|item| item.name == "operators_strict_equality")
  //     .count()
  // }

  // #[test]
  // fn should_ok_when_async_generator_function_declaration() {
  //   let mut tester =
  //     SemanticTester::from_visitor(StrictEqualityVisitor::default());
  //   let usage = tester.analyze("console.log('5' === 5);");

  //   let count = get_async_function_count(&usage);

  //   assert_eq!(usage.len(), 1);

  //   assert_eq!(count, 1);
  // }
}
