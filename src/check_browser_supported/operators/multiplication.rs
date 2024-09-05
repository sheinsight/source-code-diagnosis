use oxc_syntax::operator::BinaryOperator;

use crate::create_compat_2;

create_compat_2! {
  Multiplication,
  compat {
    name: "operators.multiplication",
    description: "乘法运算符 (*)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Multiplication",
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
    matches!(node.kind(), AstKind::BinaryExpression(expr) if expr.operator == BinaryOperator::Multiplication)
  }
}

#[cfg(test)]
mod tests {
  use super::Multiplication;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_multiplication:{
      setup: Multiplication::default(),
      source_code: r#"
        console.log(2 * 3);
        let x = 4 * 5;
      "#,
      eq: [
        r#"2 * 3"#,
        r#"4 * 5"#,
      ],
      ne: []
    }
  }
}
