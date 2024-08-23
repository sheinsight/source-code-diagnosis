use crate::create_compat;

create_compat! {
  "./try_catch.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_try_statement.push(walk_try_statement);
  },

  walk_try_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::TryStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "try_catch",
    setup,

    should_ok_when_use_try_expression,
    r#"
    try {
      nonExistentFunction();
    } catch (error) {
      console.error(error);
    }
    "#,
    1
  }
}
