use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  ClassesStatic,
  compat {
    name: "classes.static",
    description: "static class members",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/static",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "45",
      firefox_android: "45",
      safari: "9",
      safari_ios: "9",
      edge: "13",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    match node.kind() {
      AstKind::MethodDefinition(method_definition) => method_definition.r#static,
      AstKind::PropertyDefinition(property_definition) => property_definition.r#static,
      AstKind::StaticBlock(_) => true,
      _ => false,
    }
  }
}

#[cfg(test)]
mod tests {

  use super::ClassesStatic;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_static_method:{
      setup: ClassesStatic::default(),
      source_code: r#"
        class A { static a() { } }
      "#,
      eq: [
        r#"static a() { }"#,
      ],
      ne: [

      ]
    },

    should_ok_when_use_static_property:{
      setup: ClassesStatic::default(),
      source_code: r#"
        class A { static a = 1; }
      "#,
      eq: [
        r#"static a = 1;"#,
      ],
      ne: [

      ]
    },

    should_ok_when_use_static_block:{
      setup: ClassesStatic::default(),
      source_code: r#"
        class A { static { } }
      "#,
      eq: [
        r#"static { }"#,
      ],
      ne: []
    },

    should_ok_when_use_all_static:{
      setup: ClassesStatic::default(),
      source_code: r#"
        class A {
          static a() { }
          static b = 1;
          static { }
        }
      "#,
      eq: [
        r#"static a() { }"#,
        r#"static b = 1;"#,
        r#"static { }"#,
      ],
      ne: []
    }
  }
}
