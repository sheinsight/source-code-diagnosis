use crate::create_compat_2;

create_compat_2! {
  OperatorsYield,
  compat {
    name: "operators.yield",
    description: "yield 操作符",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/yield",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "39",
      chrome_android: "39",
      firefox: "26",
      firefox_android: "26",
      safari: "10",
      safari_ios: "10",
      edge: "12",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::YieldExpression(expr) if !expr.delegate)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsYield;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_yield:{
      setup: OperatorsYield::default(),
      source_code: r#"
        function* countAppleSales() {
          yield 3;
          yield 7;
          yield* 5;
        }
      "#,
      eq: [
        r#"yield 3"#,
        r#"yield 7"#,
      ],
      ne: [
        r#"yield* 5"#,
      ]
    }
  }
}
