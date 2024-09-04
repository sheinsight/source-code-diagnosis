use oxc_syntax::operator::BinaryOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsLeftShift,
  compat {
    name: "operators.left_shift",
    description: "按位左移运算符 (<<)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Left_shift",
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
    matches!(node.kind(), AstKind::BinaryExpression(expr) if matches!(expr.operator, BinaryOperator::ShiftLeft))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsLeftShift;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_left_shift:{
      setup: OperatorsLeftShift::default(),
      source_code: r#"
        console.log(9 << 2);
        let x = 5;
        x <<= 1;
      "#,
      eq: [
        r#"9 << 2"#,
      ],
      ne: [
        r#"x <<= 1"#,
      ]
    }
  }
}
