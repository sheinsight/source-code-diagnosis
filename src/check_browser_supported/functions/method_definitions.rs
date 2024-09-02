use crate::create_compat_2;

create_compat_2! {
  MethodDefinitions,
  compat {
    name: "method_definitions",
    description: "Method definitions",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Method_definitions",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "39.0.0",
      chrome_android: "39.0.0",
      firefox: "34.0.0",
      firefox_android: "34.0.0",
      safari: "9.0.0",
      safari_ios: "9.0.0",
      edge: "12.0.0",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::ObjectProperty(prop) = node.kind() {
        return prop.method;
    }

    if let AstKind::MethodDefinition(method) = node.kind() {
        return true;
    }

    false
  }
}

#[cfg(test)]
mod tests {

  use super::MethodDefinitions;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: MethodDefinitions::default(),
      source_code: r#"
        const obj = {
          foo() { return 'bar'; },
          bar: function () { },
        };
      "#,
      eq: [
        r#"foo() { return 'bar'; }"#,
      ],
      ne: []
    },

    should_ok_when_use_method_definitions_with_computed_property:{
      setup: MethodDefinitions::default(),
      source_code: r#"
        const obj = {
          [expr]() { return 'bar'; },
        };
      "#,
      eq: [
        r#"[expr]() { return 'bar'; }"#,
      ],
      ne: []
    },

    should_ok_when_use_method_definitions_with_class_declaration:{
      setup: MethodDefinitions::default(),
      source_code: r#"
        class Foo {
          foo() { return 'bar'; }
        }
      "#,
      eq: [
        r#"foo() { return 'bar'; }"#,
      ],
      ne: []
    }
  }
}
