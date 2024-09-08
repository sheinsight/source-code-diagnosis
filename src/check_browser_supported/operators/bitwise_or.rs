use oxc_ast::AstKind;
use oxc_syntax::operator::BinaryOperator;

use crate::create_compat_2;

create_compat_2! {
  BitwiseOr,
  compat {
    name: "operators.bitwise_or",
    description: "按位或运算符 (|)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Bitwise_OR",
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
    if let AstKind::BinaryExpression(binary_expr) = node.kind() {
      matches!(binary_expr.operator, BinaryOperator::BitwiseOR)
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::BitwiseOr;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_bitwise_or:{
      setup: BitwiseOr::default(),
      source_code: r#"
        console.log(5 | 3);
        let x = 10 | 4;
      "#,
      eq: [
        r#"5 | 3"#,
        r#"10 | 4"#,
      ],
      ne: []
    }
  }
}
