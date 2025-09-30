use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  ClassesExpression,
  compat {
    name: "operators.class",
    description: "The class keyword can be used to define a class inside an expression.",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/class",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "42",
      chrome_android: "42",
      firefox: "45",
      firefox_android: "45",
      safari: "7",
      safari_ios: "7",
      edge: "13",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::Class(class) if class.is_expression())
  }
}

#[cfg(test)]
mod tests {
  use super::ClassesExpression;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_expression:{
      setup: ClassesExpression::default(),
      source_code: r#"
        const Rectangle = class {
          constructor(height, width) {
            this.height = height;
            this.width = width;
          }
        };
      "#,
      eq: [
        r#"class {
          constructor(height, width) {
            this.height = height;
            this.width = width;
          }
        }"#
      ],
      ne: []
    }
  }
}
