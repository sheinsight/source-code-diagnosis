use oxc_syntax::operator::UnaryOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsUnaryNegation,
  compat {
    name: "operators.unary_negation",
    description: "一元负号运算符 (<code>-</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Unary_negation",
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
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::UnaryExpression(expr) if expr.operator == UnaryOperator::UnaryNegation)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsUnaryNegation;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_unary_negation:{
      setup: OperatorsUnaryNegation::default(),
      source_code: r#"
        let x = 5;
        console.log(-x);
        let y = -10;
      "#,
      eq: [
        r#"-x"#,
        r#"-10"#,
      ],
      ne: []
    }
  }
}
