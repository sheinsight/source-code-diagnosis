use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./exponentiation.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_binary_expression.push(walk_binary_expression);
  },

  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::Exponential)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_exponentiation",
    setup,

    should_ok_when_exponentiation,
    r#"
    console.log(2 ** 3);
    "#,
    1,
  }
}
