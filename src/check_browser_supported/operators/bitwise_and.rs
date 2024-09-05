use oxc_syntax::operator::BinaryOperator;

use crate::create_compat_2;

create_compat_2! {
  BitwiseAnd,
  compat {
    name: "operators.bitwise_and",
    description: "按位与运算符 (<code>a & b</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Bitwise_AND",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "4",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::BinaryExpression(expr) if matches!(expr.operator, BinaryOperator::BitwiseAnd))
  }
}

#[cfg(test)]
mod tests {
  use super::BitwiseAnd;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_bitwise_and:{
      setup: BitwiseAnd::default(),
      source_code: r#"
        console.log(5 & 3);
      "#,
      eq: [
        r#"5 & 3"#,
      ],
      ne: []
    }
  }
}
