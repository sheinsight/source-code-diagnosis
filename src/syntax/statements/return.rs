use crate::create_compat;

create_compat! {
  "./return.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_return_statement.push(walk_return_statement);
  },

  walk_return_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ReturnStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "return",
    setup,
    should_ok_when_return_statement,
    r#"
  function getRectArea(width, height) {
    if (width > 0 && height > 0) {
      return width * height;
    }
    return 0;
  }
    "#,
    2,
  }
}
