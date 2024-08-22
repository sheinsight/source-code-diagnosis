use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./bitwise_or.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_binary_expression.push(walk_binary_expression);
  },

  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::BitwiseOR)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_bitwise_or",
    setup,
    should_ok_when_use_bitwise_or,
    r#"
      console.log(5 | 3);
    "#,
    1
  }
}
