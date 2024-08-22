use oxc_syntax::operator::UpdateOperator;

use crate::create_compat;

create_compat! {
  "./increment.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_update_expression.push(walk_update_expression);
  },

  walk_update_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::UpdateExpression| {
    matches!(it.operator, UpdateOperator::Increment)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_increment",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    let x = 3;
    const y = x++;
    "#,
    1,
  }
}
