use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./left_shift.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_binary_expression.push(walk_binary_expression);
  },

  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::ShiftLeft)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_left_shift",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    console.log(9 << 2);
    "#,
    1,
  }
}
