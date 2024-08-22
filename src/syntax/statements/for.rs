use crate::create_compat;

create_compat! {
  "./for.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_for_statement.push(walk_for_statement);
  },
  walk_for_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ForStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "for",
    setup,
    should_ok_when_for_statement,
    r#"
        for (let i = 0; i < 9; i++) {
          console.log(i);
        }
      "#,
      1
  }
}
