use oxc_ast::{ast::Expression, AstKind};

use crate::create_compat;

create_compat! {
  Iterator,
  compat {
    name: "functions.arguments.iterator",
    description: "[Symbol.iterator]",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments/Symbol.iterator",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "52",
      chrome_android: "52",
      firefox: "46",
      firefox_android: "46",
      safari: "9",
      safari_ios: "9",
      edge: "12",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::ForOfStatement(for_of_statement) = node.kind() {
        if let Expression::Identifier(identifier) = &for_of_statement.right {
            return identifier.name == "arguments"
        }
    }
    false
  }
}

#[cfg(test)]
mod tests {

  use super::Iterator;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: Iterator::default(),
      source_code: r#"
        function f() {
          for (const letter of arguments) {}
        }
        f("w", "y", "k", "o", "p");
      "#,
      eq: [
        r#"for (const letter of arguments) {}"#,
      ],
      ne: []
    },
    should_ok_when_use_arguments_length:{
      setup: Iterator::default(),
      source_code: r#"
         const f = (arguments) => {
            for (const letter of arguments) { }
          }
          f("w", "y", "k", "o", "p");
      "#,
      eq: [
        r#"for (const letter of arguments) { }"#,
      ],
      ne: []
    }

  }
}
