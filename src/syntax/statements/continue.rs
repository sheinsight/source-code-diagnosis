use crate::create_compat;

create_compat! {
  "./continue.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_continue_statement.push(walk_continue_statement);
  },
  walk_continue_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ContinueStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "continue",
    setup,
    should_ok_when_for_statement,
    r#"
    for (let i = 0; i < 10; i++) {
      if (i === 3) {
        continue;
      }
      text = text + i;
    }
    "#,
    1,
  }
}
