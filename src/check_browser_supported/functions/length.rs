use oxc_ast::ast::{Expression, MemberExpression};

use crate::create_compat_2;

create_compat_2! {
  Length,
  compat {
    name: "functions.arguments.length",
    description: "function arguments length",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments/length",
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
    if let AstKind::MemberExpression(member_expression) = node.kind() {
      if let MemberExpression::StaticMemberExpression(static_member_expression) = member_expression {
        if let Expression::Identifier(identifier) = &static_member_expression.object {
          return identifier.name == "arguments" && static_member_expression.property.name == "length"
        }
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {

  use super::Length;
  use crate::assert_source_seg;

  // TODO

  /**
  *
    function func1(a, b, c) {
      let len = arguments;
      console.log(len.length);
    }
  */

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: Length::default(),
      source_code: r#"
        function func1(a, b, c) {
            console.log(arguments.length);
        }
        func1(1, 2, 3);
      "#,
      eq: [
        r#"arguments.length"#,
      ],
      ne: []
    },
    should_ok_when_use_arguments_length:{
      setup: Length::default(),
      source_code: r#"
        function func1(a, b, c) {
            let len = arguments.length;
            console.log(len);
        }
        func1(1, 2, 3);
      "#,
      eq: [
        r#"arguments.length"#,
      ],
      ne: []
    }

  }
}
