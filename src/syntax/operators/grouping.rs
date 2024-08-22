use crate::create_compat;

create_compat! {
  "./grouping.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_parenthesized_expression.push(walk_parenthesized_expression);
  },

  walk_parenthesized_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ParenthesizedExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_grouping",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    console.log((2 + 3) * 4);
    "#,
    1,
  }
}
