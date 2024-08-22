use crate::create_compat;

create_compat! {
  "./conditional.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_conditional_expression.push(walk_conditional_expression);
  },

  walk_conditional_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ConditionalExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_conditional",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
      const age = 26;
      const beverage = age >= 21 ? "Beer" : "Juice";
      console.log(beverage);
    "#,
    1
  }
}
