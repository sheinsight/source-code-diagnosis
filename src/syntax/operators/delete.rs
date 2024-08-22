use oxc_syntax::operator::UnaryOperator;

use crate::create_compat;

create_compat! {
  "./delete.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_unary_expression.push(walk_unary_expression);
  },

  walk_unary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::UnaryExpression| {
    matches!(it.operator, UnaryOperator::Delete)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "delete",
    setup,


    should_ok_when_delete_object_key,
    r#"
      const myObject = {x: 1, y: 2};
      delete myObject.x;
    "#,
    1,

  }
}
