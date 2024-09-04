use oxc_syntax::operator::UnaryOperator;

use crate::create_compat_2;

create_compat_2! {
  BitwiseNot,
  compat {
    name: "operators_bitwise_not",
    description: "按位非运算符 (<code>~a</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Bitwise_NOT",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1.0.0",
      chrome_android: "18.0.0",
      firefox: "1.0.0",
      firefox_android: "4.0.0",
      safari: "1.0.0",
      safari_ios: "1.0.0",
      edge: "12.0.0",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::UnaryExpression(expr) if matches!(expr.operator, UnaryOperator::BitwiseNot))
  }
}

#[cfg(test)]
mod tests {
  use super::BitwiseNot;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_bitwise_not:{
      setup: BitwiseNot::default(),
      source_code: r#"
        console.log(~5);
        console.log(~(5));
      "#,
      eq: [
        r#"~5"#,
        r#"~(5)"#,
      ],
      ne: []
    }
  }
}
