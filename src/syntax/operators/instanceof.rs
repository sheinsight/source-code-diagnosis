use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./instanceof.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_binary_expression.push(walk_binary_expression);
  },

  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::Instanceof)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::setup;

  assert_ok_count! {
    "operators_instanceof",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    console.log(auto instanceof Car);
    "#,
    1,
  }
}
