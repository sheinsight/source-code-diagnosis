use oxc_ast::ast::PropertyDefinition;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_property_definition.push(walk_property_definition);
  },
  compat {
    name: "classes_static_class_fields",
    description: "static class fields",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2022"
    ],
    support: {
      chrome: "72",
      chrome_android: "72",
      firefox: "75",
      firefox_android: "79",
      safari: "14.1",
      safari_ios: "14.5",
      edge: "79",
      node: "12.0.0",
      deno: "1.0",
    }
  },
  walk_property_definition,
  |ctx: &mut Context, it: &PropertyDefinition| {
    it.r#static
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "classes_static_class_fields",
    setup,

    should_ok_when_use_class_static_fields,
    r#"
      class A { static hello = 12 }
    "#,
    1,

    should_ok_when_use_class_static_fields_and_instance_static_fields,
    r#"
      class A { static hello = 12; hello = 12 }
    "#,
    1,

    should_ok_when_use_class_instance_fields,
    r#"
      class A { hello = 12 }
    "#,
    0,

    should_ok_when_use_class_instance_fields_and_instance_static_fields,
    r#"
      class A { hello = 12; static hello = 12 }
    "#,
    1,
  }
}
