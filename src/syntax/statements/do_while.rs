use crate::create_compat;

create_compat! {
  "./do_while.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_do_while_statement.push(walk_do_while_statement);
  },
  walk_do_while_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::DoWhileStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "do_while",
    setup,
    should_ok_when_do_while_statement,
    r#"
    do {
      i = i + 1;
      result = result + i;
    } while (i < 5);
    "#,
    1,
  }
}
