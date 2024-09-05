use crate::create_compat_2;

create_compat_2! {
  SpreadInFunctionCalls,
  compat {
    name: "operators.spread.spread_in_function_calls",
    description: "函数调用中的展开语法",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Spread_syntax#spread_in_function_calls",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "46",
      chrome_android: "46",
      firefox: "27",
      firefox_android: "27",
      safari: "8",
      safari_ios: "8",
      edge: "12",
      node: "5.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, nodes: &AstNodes<'a>) -> bool {
    if let AstKind::SpreadElement(_) = node.kind() {
        if let Some(parent_node) = nodes.parent_node(node.id()) {
            return matches!(parent_node.kind(), AstKind::Argument(_));
        }
    }
    false

  }
}

#[cfg(test)]
mod tests {
  use super::SpreadInFunctionCalls;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_spread_in_function_calls:{
      setup: SpreadInFunctionCalls::default(),
      source_code: r#"
        function myFunction(x, y, z) {}
        const args = [0, 1, 2];
        myFunction(...args);     
      "#,
      eq: [
        r#"...args"#,
      ],
      ne: []
    },

    should_ok_when_use_multiple_spread_in_function_calls:{
      setup: SpreadInFunctionCalls::default(),
      source_code: r#"
        function myFunction(x, y, z) {}
        const args1 = [0, 1];
        const arg2 = 2;
        myFunction(...args1, arg2, ...['a', 'b']);     
      "#,
      eq: [
        r#"...args1"#,
        r#"...['a', 'b']"#,
      ],
      ne: []
    },

    should_not_ok_when_not_use_spread_in_function_calls:{
      setup: SpreadInFunctionCalls::default(),
      source_code: r#"
        function myFunction(x, y, z) {}
        const args = [0, 1, 2];
        myFunction(args);     
      "#,
      eq: [],
      ne: [
        r#"myFunction(args)"#,
      ]
    }
  }
}
