use oxc_syntax::operator::LogicalOperator;

use crate::create_compat;

create_compat! {
  "./nullish_coalescing.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_logical_expression.push(walk_logical_expression);
  },

  walk_logical_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::LogicalExpression| {
    matches!(it.operator, LogicalOperator::Coalesce)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_nullish_coalescing",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    const foo = null ?? 'default string';
    console.log(foo);
    "#,
    1,
  }
}
