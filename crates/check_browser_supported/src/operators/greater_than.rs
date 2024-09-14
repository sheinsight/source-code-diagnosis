use oxc_ast::AstKind;
use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  OperatorsGreaterThan,
  compat {
    name: "operators.greater_than",
    description: "大于运算符 (>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Greater_than",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "18",
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
    matches!(node.kind(), AstKind::BinaryExpression(expr) if expr.operator == BinaryOperator::GreaterThan)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsGreaterThan;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_greater_than:{
      setup: OperatorsGreaterThan::default(),
      source_code: r#"
        console.log(5 > 3);
        console.log(10 > 5);
      "#,
      eq: [
        r#"5 > 3"#,
        r#"10 > 5"#,
      ],
      ne: []
    }
  }
}
