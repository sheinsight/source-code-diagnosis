use crate::create_compat;

create_compat! {
  "./destructuring_rest_in_objects.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_pattern.push(walk_object_pattern);
  },

  walk_object_pattern,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectPattern| {
    it.rest.is_some()
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "rest_in_objects",
    setup,

    should_ok_when_use_rest_in_objects,
    r#"
      const {a, ...b} = object;
    "#,
    1,
  }
}
