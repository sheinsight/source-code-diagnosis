use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  "./addition.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_binary_expression.push(walk_binary_expression);
  },

  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::Addition)
  }
}

#[cfg(test)]
mod tests {
  use crate::{assert_ok_count, syntax::operators::addition::setup};

  assert_ok_count! {
    "operators_addition",
    setup,

    should_ok_when_use_addition,
    r#"
      console.log(2 + 2);
    "#,
    1,
  }
}
