use crate::create_compat;

create_compat! {
  "./class.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_class.push(walk_class);
  },

  walk_class,
  |ctx: &mut Context, it: &oxc_ast::ast::Class| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "class",
    setup,

    should_ok_when_class_declaration,
    r#"
    class A {}
    "#,
    1,
  }
}
