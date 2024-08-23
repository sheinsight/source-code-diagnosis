use oxc_ast::ast::{MethodDefinition, PropertyKey};

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_method_definition.push(walk_method_definition);
  },
  compat {
    name: "classes_private_class_methods",
    description: "private class methods",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2022"
    ],
    support: {
      chrome: "74",
      chrome_android: "74",
      firefox: "90",
      firefox_android: "90",
      opera: "62",
      opera_android: "53",
      safari: "14.1",
      safari_ios: "14.5",
      edge: "79",
      oculus: "74",
      node: "12.0.0",
      deno: "1.0",
    }
  },
  walk_method_definition,
  |ctx: &mut Context, it: &MethodDefinition| {
    matches!(it.key, PropertyKey::PrivateIdentifier(_))
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "classes_private_class_methods",
    setup,

    should_ok_when_use_private_class_methods,
    r#"
      class A {
        #private_method(){}
      }
    "#,
    1,

    should_ok_when_not_use_private_class_methods,
    r#"
      class A {
        private_method(){}
      }
    "#,
    0
  }
}
