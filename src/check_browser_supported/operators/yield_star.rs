use oxc_ast::AstKind;

use crate::create_compat_2;

create_compat_2! {
  OperatorsYieldStar,
  compat {
    name: "operators.yield_star",
    description: "yield* 表达式",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/yield*",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "39",
      chrome_android: "39",
      firefox: "27",
      firefox_android: "27",
      safari: "10",
      safari_ios: "10",
      edge: "12",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::YieldExpression(yield_expr) if yield_expr.delegate)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsYieldStar;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_yield_star:{
      setup: OperatorsYieldStar::default(),
      source_code: r#"
        function* generatorB() {
          yield 'Generator B:';
          yield* generatorA();
          yield 'End of B';
        }
      "#,
      eq: [
        r#"yield* generatorA()"#,
      ],
      ne: [
        r#"yield 'Generator B:'"#,
        r#"yield 'End of B'"#,
      ]
    }
  }
}
