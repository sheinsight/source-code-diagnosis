use crate::create_compat;

create_compat! {
  "./throw.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_throw_statement.push(walk_throw_statement);
  },

  walk_throw_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ThrowStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "throw",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    throw new Error('Parameter is not a number!');
    "#,
    1,
  }
}
