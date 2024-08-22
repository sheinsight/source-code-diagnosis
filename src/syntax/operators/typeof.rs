use oxc_syntax::operator::UnaryOperator;

use crate::create_compat;

create_compat! {
  "./typeof.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_unary_expression.push(walk_unary_expression);
  },

  walk_unary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::UnaryExpression| {
    matches!(it.operator, UnaryOperator::Typeof)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_typeof",
    setup,

    should_ok_when_typeof_number,
    r#"typeof 42"#,
    1,

  }
}
