use crate::create_compat;

create_compat! {
  "./class.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_class.push(walk_class);
  },

  walk_class,
  |ctx: &mut Context, it: &oxc_ast::ast::Class| {
    it.is_expression()
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_class",
    setup,

    should_ok_when_use_class,
    r#"
      const Rectangle = class {
        constructor(height, width) {
          this.height = height;
          this.width = width;
        }
      };
    "#,
    1
  }
}
