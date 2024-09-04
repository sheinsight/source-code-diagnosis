use crate::create_compat_2;

create_compat_2! {
  OperatorsFunctionExpression,
  compat {
    name: "operators_function_expression",
    description: "<code>function</code> 表达式",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/function",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1.0.0",
      chrome_android: "1.0.0",
      firefox: "1.0.0",
      firefox_android: "1.0.0",
      safari: "1.0.0",
      safari_ios: "1.0.0",
      edge: "12.0.0",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::Function(_))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsFunctionExpression;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_function_expression:{
      setup: OperatorsFunctionExpression::default(),
      source_code: r#"
        let greet = function(name) {
          console.log("Hello, " + name);
        };
        greet("Alice");
      "#,
      eq: [
        r#"function(name) {
          console.log("Hello, " + name);
        }"#,
      ],
      ne: []
    },

    should_ok_when_iife:{
      setup: OperatorsFunctionExpression::default(),
      source_code: r#"
        (function() {
          console.log("This function is executed immediately.");
        })();
      "#,
      eq: [
        r#"function() {
          console.log("This function is executed immediately.");
        }"#,
      ],
      ne: []
    }
  }
}
