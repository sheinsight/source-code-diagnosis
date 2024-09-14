use oxc_ast::AstKind;
use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  OperatorsUnsignedRightShift,
  compat {
    name: "operators.unsigned_right_shift",
    description: "无符号右移运算符 (<code>a >>> b</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Unsigned_right_shift",
    tags: ["web-features:snapshot:ecmascript-1"],
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
    matches!(node.kind(), AstKind::BinaryExpression(expr) if matches!(expr.operator, BinaryOperator::ShiftRightZeroFill))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsUnsignedRightShift;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_unsigned_right_shift:{
      setup: OperatorsUnsignedRightShift::default(),
      source_code: r#"
        console.log(8 >>> 2);
        let x = 16;
        x >>>= 3;
      "#,
      eq: [
        r#"8 >>> 2"#,
      ],
      ne: [
        r#"x >>>= 3"#,
      ]
    }
  }
}
