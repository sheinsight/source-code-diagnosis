use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./remainder.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_binary_expression.push(walk_binary_expression);
  },
  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::Remainder)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_remainder",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
      console.log(10 % 3);
    "#,
    1,
  }
}
