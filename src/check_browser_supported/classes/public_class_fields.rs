use oxc_ast::ast::{PropertyDefinition, PropertyKey};

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_property_definition.push(walk_property_definition);
  },
  compat {
    name: "classes_public_class_fields",
    description: "public class fields",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2022"
    ],
    support: {
      chrome: "72",
      chrome_android: "72",
      firefox: "69",
      firefox_android: "69",
      safari: "14.1",
      safari_ios: "14.5",
      edge: "79",
      node: "12.0.0",
      deno: "1.0",
    }
  },
  walk_property_definition,
  |ctx: &mut Context, it: &PropertyDefinition| {
    !matches!(it.key, PropertyKey::PrivateIdentifier(_))
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "classes_public_class_fields",
    setup,

    should_ok_when_use_class_public_fields,
    r#"
      class A { hello = 12 }
    "#,
    1,

    should_ok_when_use_class_static_public_fields,
    r#"
      class A { static hello = 12 }
    "#,
    1,

    should_ok_when_use_class_public_fields_and_static_public_fields,
    r#"
      class A { hello = 12; static hello = 12 }
    "#,
    2,

    should_ok_when_use_class_private_fields,
    r#"
      class A { #hello = 12 }
    "#,
    0,

    should_ok_when_use_class_private_fields_and_static_private_fields,
    r#"
      class A { #hello = 12; static #hello = 12 }
    "#,
    0,

    should_ok_when_use_class_public_fields_and_private_fields,
    r#"
      class A { hello = 12; #hello = 12 }
    "#,
    1,
  }
}
