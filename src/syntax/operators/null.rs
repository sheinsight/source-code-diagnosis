use crate::create_compat;

create_compat! {
  "./null.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_null_literal.push(walk_null_literal);
  },

  walk_null_literal,
  |ctx: &mut Context, it: &oxc_ast::ast::NullLiteral| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_null",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    let obj = null;
    "#,
    1,
  }
}
