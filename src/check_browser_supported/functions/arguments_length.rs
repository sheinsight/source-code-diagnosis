use oxc_ast::ast::{Expression, StaticMemberExpression};

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_static_member_expression.push(arguments_length);
  },
  compat {
    name: "arguments_length",
    description: "arguments.length",
    tags: [
      "web-features:arguments-length",
      "web-features:snapshot:ecmascript-5"
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
      deno: "1.0",
    }
  },
  arguments_length,
  |ctx: &mut Context, node: &StaticMemberExpression| {
    if let Expression::Identifier(o) = &node.object {
      o.name == "arguments" && node.property.name == "length"
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
    "arguments_length",
    setup,

    should_ok_when_use_arguments_length,
    r#"
      function logArguments() {
          console.log(` ${arguments.length} `);
          for (let i = 0; i < arguments.length; i++) {
              console.log(`${i + 1}: ${arguments[i]}`);
          }
      }
    "#,
    2,

    should_ok_when_not_use_arguments_length,
    r#"
      function logArguments() {
          
      }
    "#,
    0,

    should_ok_when_use_arguments_length_in_arrow_function,
    r#"
      const logArguments = () => {
          console.log(` ${arguments.length} `);
          for (let i = 0; i < arguments.length; i++) {
              console.log(`${i + 1}: ${arguments[i]}`);
          }
      }
    "#,
    2,
  }
}
