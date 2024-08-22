use crate::create_compat;

create_compat! {
  "./empty.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_empty_statement.push(walk_empty_statement);
  },
  walk_empty_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::EmptyStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "empty",
    setup,
    should_ok_when_empty_statement,
    r#"
    ;
    "#,
    1,
  }
}
