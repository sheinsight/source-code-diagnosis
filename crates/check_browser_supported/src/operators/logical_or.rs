use oxc::ast::AstKind;
use oxc::syntax::operator::LogicalOperator;

use crate::create_compat;

create_compat! {
  OperatorsLogicalOr,
  compat {
    name: "operators.logical_or",
    description: "逻辑或运算符 (||)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Logical_OR",
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
    matches!(node.kind(), AstKind::LogicalExpression(expr) if expr.operator == LogicalOperator::Or)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsLogicalOr;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_logical_or:{
      setup: OperatorsLogicalOr::default(),
      source_code: r#"
        console.log(true || false);
        let x = a || b;
      "#,
      eq: [
        r#"true || false"#,
        r#"a || b"#,
      ],
      ne: []
    }
  }
}
