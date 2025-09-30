use oxc::ast::{ast::PropertyKey, AstKind};

use crate::create_compat;

create_compat! {
  ClassesPublicClassFields,
  compat {
    name: "classes.public_class_fields",
    description: "public class fields",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/Public_class_fields",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "72",
      chrome_android: "72",
      firefox: "69",
      firefox_android: "69",
      safari: "16",
      safari_ios: "16",
      edge: "79",
      node: "12.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(
      node.kind(), AstKind::PropertyDefinition(property_definition)
      if !matches!(property_definition.key, PropertyKey::PrivateIdentifier(_))
      && !property_definition.r#static
    )
  }
}

#[cfg(test)]
mod tests {

  use super::ClassesPublicClassFields;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_private_in_expression:{
      setup: ClassesPublicClassFields::default(),
      source_code: r#"
        class C {
          name = "hello";
          static hello = 12;
          #x;
          constructor(x) {
            this.#x = x;
          }
          getX(obj) {
            if (#x in obj) return obj.#x;
            return "obj must be an instance of C";
          }
        }
      "#,
      eq: [
        r#"name = "hello";"#
      ],
      ne: [
        r#"#x;"#,
        r#"static hello = 12;"#
      ]
    },
  }
}
