use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./division.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_binary_expression.push(walk_binary_expression);
  },

  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::Division)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "division",
    setup,

    should_ok_when_division,
    r#"
    console.log(6 / 2);
    "#,
    1
  }
}
