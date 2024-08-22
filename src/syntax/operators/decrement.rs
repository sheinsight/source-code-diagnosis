use oxc_syntax::operator::UpdateOperator;

use crate::create_compat;

create_compat! {
  "./decrement.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_update_expression.push(walk_update_expression);
  },

  walk_update_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::UpdateExpression| {
    matches!(it.operator, UpdateOperator::Decrement)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_decrement",
    setup,
    should_ok_when_use_decrement,
    r#"
      let x = 3;
      let y = x--;
      console.log(x);
      console.log(y);
    "#,
    1,


  }
}
