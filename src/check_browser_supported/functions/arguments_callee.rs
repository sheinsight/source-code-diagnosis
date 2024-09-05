use oxc_ast::ast::{Expression, MemberExpression};

use crate::create_compat_2;

create_compat_2! {
  Callee,
  compat {
    name: "functions.arguments.callee",
    description: "function arguments callee",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments/callee",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::CallExpression(call_expression) = node.kind() {
      if let Some(callee) = call_expression.callee.as_member_expression() {
        if let MemberExpression::StaticMemberExpression(static_member_expression) = callee {
          if let Expression::Identifier(identifier) = &static_member_expression.object {
            return identifier.name == "arguments" && static_member_expression.property.name == "callee"
          }
        }
      }

    }
    false
  }
}

#[cfg(test)]
mod tests {

  use super::Callee;
  use crate::assert_source_seg;

  // TODO
  /**
  * [1, 2, 3, 4, 5].map(function (n) {
      let a = arguments.callee;
     return n <= 1 ? 1 : a(n - 1) * n;
   });
  */

  /**
  * [1, 2, 3, 4, 5].map(function (n) {
      let a = arguments;
     return n <= 1 ? 1 : a.callee(n - 1) * n;
   });
  */

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: Callee::default(),
      source_code: r#"
        [1, 2, 3, 4, 5].map(function (n) {
          return n <= 1 ? 1 : arguments.callee(n - 1) * n;
        });
      "#,
      eq: [
        r#"arguments.callee(n - 1)"#,
      ],
      ne: [
        r#"function() { }"#
      ]
    }

  }
}
