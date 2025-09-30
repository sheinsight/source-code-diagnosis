use oxc::ast::AstKind;
use oxc::syntax::operator::LogicalOperator;

use crate::create_compat;

create_compat! {
  OperatorsLogicalAnd,
  compat {
    name: "operators.logical_and",
    description: "逻辑与运算符 (&&)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Logical_AND",
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
    matches!(node.kind(), AstKind::LogicalExpression(expr) if expr.operator == LogicalOperator::And)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsLogicalAnd;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_logical_and:{
      setup: OperatorsLogicalAnd::default(),
      source_code: r#"
        console.log(true && true);
        console.log(false && true);
      "#,
      eq: [
        r#"true && true"#,
        r#"false && true"#,
      ],
      ne: []
    }
  }
}
