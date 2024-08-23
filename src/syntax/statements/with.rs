use crate::create_compat;

create_compat! {
  "./with.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_with_statement.push(walk_with_statement);
  },

  walk_with_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::WithStatement| {
    true
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "with",
    setup,

    should_ok_when_use_with_statement,
    r#"
    with ([1, 2, 3]) {
      console.log(toString()); // 1,2,3
    }    
    "#,
    1
  }
}
