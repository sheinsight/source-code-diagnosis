use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  ClassesPrivateClassFieldsIn,
  compat {
    name: "classes.private_class_fields_in",
    description: "private class fields 'in' operator",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/Private_properties#javascript.classes.private_class_fields_in",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "91",
      chrome_android: "91",
      firefox: "90",
      firefox_android: "90",
      safari: "15",
      safari_ios: "15",
      edge: "91",
      node: "16.4.0",
      deno: "1.9.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(),AstKind::PrivateInExpression(_))
  }
}

#[cfg(test)]
mod tests {

  use super::ClassesPrivateClassFieldsIn;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_private_in_expression:{
      setup: ClassesPrivateClassFieldsIn::default(),
      source_code: r#"
        class C {
          #x;
          constructor(x) {
            this.#x = x;
          }
          static getX(obj) {
            if (#x in obj) return obj.#x;
            return "obj must be an instance of C";
          }
        }
      "#,
      eq: [
        r#"#x in obj"#
      ],
      ne: []
    }
  }
}
