use crate::create_compat;

create_compat! {
  "./destructuring_rest_in_arrays.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_array_pattern.push(walk_array_pattern);
  },

  walk_array_pattern,
  |ctx: &mut Context, it: &oxc_ast::ast::ArrayPattern| {
    it.rest.is_some()
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "rest_in_arrays",
    setup,

    should_ok_when_use_rest_in_arrays,
    r#"
      const [a, ...b] = array;
    "#,
    1,
  }
}
