use crate::create_compat;

create_compat! {
  "./destructuring_computed_property_names.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_pattern.push(walk_object_pattern);
  },

  walk_object_pattern,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectPattern| {
    it.properties.iter().any(|prop| prop.computed)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "computed_property_names",
    setup,

    should_ok_when_use_computed_property_names,
    r#"
      const key = "z";
      const { [key]: a } = obj;
    "#,
    1,
  }
}
