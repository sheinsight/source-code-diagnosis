use crate::create_compat;

create_compat! {
  "./new_target.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_meta_property.push(walk_meta_property);
  },

  walk_meta_property,
  |ctx: &mut Context, it: &oxc_ast::ast::MetaProperty| {
    it.meta.name == "new" && it.property.name == "target"
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_new_target",
    setup,

    should_ok_when_async_generator_function_declaration,
    r#"
    function Foo() {
      if (!new.target) {
        throw new Error("Foo() must be called with new");
      }
      console.log("Foo instantiated with new");
    }
    "#,
    1,
  }
}
