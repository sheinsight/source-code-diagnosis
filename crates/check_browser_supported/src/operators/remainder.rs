use oxc::ast::AstKind;
use oxc::syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  OperatorsRemainder,
  compat {
    name: "operators.remainder",
    description: "余数运算符 (<code>%</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Remainder",
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
    matches!(node.kind(), AstKind::BinaryExpression(expr) if expr.operator == BinaryOperator::Remainder)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsRemainder;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_remainder_operator:{
      setup: OperatorsRemainder::default(),
      source_code: r#"
        console.log(10 % 3);
        let x = 15 % 4;
      "#,
      eq: [
        r#"10 % 3"#,
        r#"15 % 4"#,
      ],
      ne: []
    }
  }
}
