use oxc_ast::ast::{MethodDefinition, PropertyDefinition};

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_method_definition.push(walk_method_definition);
    v.walk_property_definition.push(walk_property_definition);
    v.walk_static_block.push(walk_static_block);
  },
  compat {
    name: "classes_static",
    description: "static class members",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "45",
      firefox_android: "45",
      opera: "49",
      opera_android: "49",
      safari: "9",
      safari_ios: "9",
      edge: "13",
      oculus: "49",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_method_definition,
  |ctx: &mut Context, it: &MethodDefinition| {
    it.r#static
  },
  walk_property_definition,
  |ctx: &mut Context, it: &PropertyDefinition| {
    it.r#static
  },
  walk_static_block,
  |ctx: &mut Context, it: &oxc_ast::ast::StaticBlock| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "classes_static",
    setup,

    should_ok_when_use_static_method,
    r#"
      class A { static a() { } }
    "#,
    1,

    should_ok_when_use_static_property,
    r#"
      class A { static a = 1; }
    "#,
    1,

    should_ok_when_use_static_block,
    r#"
      class A { static { } }
    "#,
    1,

    should_ok_when_use_all_static,
    r#"
      class A {
        static a() { }
        static b = 1;
        static { }
      }
    "#,
    3,

    should_ok_when_use_two_static_method,
    r#"
      class A { static a() { } static b() { } }
    "#,
    2,

    should_ok_when_use_two_static_property,
    r#"
      class A { static a = 1; static b = 2; }
    "#,
    2,

    should_ok_when_use_two_static_block,
    r#"
      class A { static { } static { } }
    "#,
    2,

    should_ok_when_not_use_static,
    r#"
      class A { }
    "#,
    0,

    should_ok_when_use_static_and_not_use_static,
    r#"
      class A {
        static a() { }
        static b = 1;
        static { }
      }
      class B { }
    "#,
    3,
  }
}
