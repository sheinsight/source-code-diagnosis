use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./inequality.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_binary_expression.push(walk_binary_expression);
  },

  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::Inequality)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_inequality",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    console.log(1 != 2);
    "#,
    1,
  }
}
