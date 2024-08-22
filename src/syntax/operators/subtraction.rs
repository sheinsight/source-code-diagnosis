use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./subtraction.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_binary_expression.push(walk_binary_expression);
  },
  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::Subtraction)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_subtraction",
    setup,
    should_ok_when_subtraction,
    r#"
      console.log(5 - 3);
    "#,
    1
  }
}
