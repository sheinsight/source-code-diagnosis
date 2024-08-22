use crate::create_compat;

create_compat! {
  "./object_initializer_shorthand_property_names.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_property.push(walk_object_property);
  },

  walk_object_property,
  |ctx: &mut Context,  it: &oxc_ast::ast::ObjectProperty| {
    it.shorthand
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_object_initializer_shorthand_property_names",
    setup,

    should_ok_when_async_generator_function_declaration,
    r#"
    const x = 10;
    const y = 20;

    const point = {
      x,
      y,
      z: 30
    };
    "#,
    2
  }
}
