use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_meta_property.push(walk_meta_property);
  },
  compat {
    name: "operators_new_target",
    description: "<code>new.target</code>",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "46",
      chrome_android: "46",
      firefox: "41",
      firefox_android: "41",
      safari: "11",
      safari_ios: "11",
      edge: "13",
      node: "5.0.0",
      deno: "1.0",
    }
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
