use oxc_ast::ast::{Expression, StaticMemberExpression};

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_static_member_expression.push(arguments_callee);
  },
  compat {
    name: "arguments_callee",
    description: "arguments.callee",
    tags: [
      "web-features:arguments-callee",
      "web-features:snapshot:ecmascript-5"
    ],
    support: {
      chrome: "1",
      chrome_android: "18",
      firefox: "1",
      firefox_android: "4",
      opera: "7",
      opera_android: "10.1",
      safari: "3",
      safari_ios: "1",
      edge: "12",
      oculus: "5.0",
      node: "0.1.100",
      deno: "1.0",
    }
  },
  arguments_callee,
  |ctx: &mut Context, node: &StaticMemberExpression| {
    if let Expression::Identifier(o) = &node.object {
      o.name == "arguments" && node.property.name == "callee"
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "arguments_callee",
    setup,

    should_ok_when_use_arguments_callee,
    r#"
      function foo() {
        arguments.callee;
      }
    "#,
    1,

    should_not_ok_when_not_use_arguments_callee,
    r#"
      function foo() {
        arguments.length;
      }
    "#,
    0,
  }
}
