use oxc_syntax::operator::UnaryOperator;

use crate::create_compat;

create_compat! {
  "./logical_not.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_unary_expression.push(walk_unary_expression);
  },

  walk_unary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::UnaryExpression| {
    matches!(it.operator, UnaryOperator::LogicalNot)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::setup;

  assert_ok_count! {
    "operators_logical_not",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    console.log(!true);
    "#,
    1,
  }
}
