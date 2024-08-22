use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./strict_inequality.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_binary_expression.push(walk_binary_expression);
  },

  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::StrictInequality)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_strict_inequality",
    setup,
    should_ok_when_strict_inequality,
    r#"
      console.log(5 !== 5);
    "#,
    1
  }
}
