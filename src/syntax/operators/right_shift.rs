use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./right_shift.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_binary_expression.push(walk_binary_expression);
  },
  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::ShiftRight)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_right_shift",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
      console.log(8 >> 1);
    "#,
    1,
  }
}
