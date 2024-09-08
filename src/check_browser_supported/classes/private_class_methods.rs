use oxc_ast::AstKind;

use crate::create_compat_2;

create_compat_2! {
  ClassesPrivateClassMethods,
  compat {
    name: "classes.private_class_methods",
    description: "private class methods",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/Private_properties#javascript.classes.private_class_methods",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "84",
      chrome_android: "84",
      firefox: "90",
      firefox_android: "90",
      safari: "15",
      safari_ios: "15",
      edge: "84",
      node: "14.6.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::MethodDefinition(method_definition) = node.kind() {
        if method_definition.key.is_private_identifier() {
            return true;
        }
    }

    false
  }
}

#[cfg(test)]
mod tests {

  use super::ClassesPrivateClassMethods;
  use crate::assert_source_seg;

  assert_source_seg! {
    test_private_class_fields:{
      setup: ClassesPrivateClassMethods::default(),
      source_code: r#"
        class MyClass {
            #privateField = 42;
            #privateMethod() { return "Hello, world!"; }
        }
      "#,
      eq: [
        r#"#privateMethod() { return "Hello, world!"; }"#,
      ],
      ne: [
        r#"#privateField = 42;"#,
      ]
    },
  }
}
