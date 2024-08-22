use crate::create_compat;

create_compat! {
  "./import_meta.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_meta_property.push(walk_meta_property);
  },

  walk_meta_property,
  |ctx: &mut Context, it: &oxc_ast::ast::MetaProperty| {
    it.meta.name == "import" && it.property.name == "meta"
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_import_meta",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    const relativeURL = import.meta.url;
    "#,
    1,
  }
}
