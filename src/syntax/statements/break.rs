use crate::create_compat;

create_compat! {
  "./break.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_break_statement.push(walk_break_statement);
  },

  walk_break_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::BreakStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "break",
    setup,

    should_ok_when_while,
    r#"
  let i = 0;

  while (i < 6) {
    if (i === 3) {
      break;
    }
    i = i + 1;
  }

  console.log(i);
    "#,
    1,
  }
}
