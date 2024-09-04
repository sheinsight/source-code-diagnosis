use oxc_syntax::operator::BinaryOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsInequality,
  compat {
    name: "operators.inequality",
    description: "不等运算符 (!=)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Inequality",
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
    matches!(node.kind(), AstKind::BinaryExpression(expr) if expr.operator == BinaryOperator::Inequality)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsInequality;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_inequality_operator:{
      setup: OperatorsInequality::default(),
      source_code: r#"
        console.log(1 != 2);
        console.log("a" != "b");
      "#,
      eq: [
        r#"1 != 2"#,
        r#""a" != "b""#,
      ],
      ne: []
    }
  }
}
