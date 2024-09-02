use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_meta_property.push(walk_meta_property);
  },
  compat {
    name: "operators_import_meta",
    description: "<code>import.meta</code>",
    tags: [],
    support: {
      chrome: "64",
      chrome_android: "64",
      firefox: "62",
      firefox_android: "62",
      safari: "11.1",
      safari_ios: "12",
      edge: "64",
      node: "10.4.0",
      deno: "1.0",
    }
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
