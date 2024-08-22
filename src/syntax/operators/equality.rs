use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./equality.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_binary_expression.push(walk_binary_expression);
  },

  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::Equality)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_equality",
    setup,

    should_ok_when_equality,
    r#"
    console.log(1 == 1);
    "#,
    1,
  }
}
