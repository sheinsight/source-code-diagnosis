use crate::create_compat;

create_compat! {
  "./this.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_this_expression.push(walk_this_expression);
  },

  walk_this_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ThisExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_this",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    console.log(this === window);
    "#,
    1
  }
}
