use oxc_ast::ast::Expression;

use crate::create_compat_2;

create_compat_2! {
  Iterator,
  compat {
    name: "functions.arguments.iterator",
    description: "[Symbol.iterator]",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments/Symbol.iterator",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "52.0.0",
      chrome_android: "52.0.0",
      firefox: "46.0.0",
      firefox_android: "46.0.0",
      safari: "9.0.0",
      safari_ios: "9.0.0",
      edge: "12.0.0",
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
