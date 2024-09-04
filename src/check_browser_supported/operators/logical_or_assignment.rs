use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsLogicalOrAssignment,
  compat {
    name: "operators_logical_or_assignment",
    description: "逻辑或赋值运算符 (x ||= y)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Logical_OR_assignment",
    tags: ["web-features:snapshot:ecmascript-2021"],
    support: {
      chrome: "85.0.0",
      chrome_android: "85.0.0",
      firefox: "79.0.0",
      firefox_android: "79.0.0",
      safari: "14.0.0",
      safari_ios: "14.0.0",
      edge: "85.0.0",
      node: "15.0.0",
      deno: "1.2.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::AssignmentExpression(expr) if matches!(expr.operator, AssignmentOperator::LogicalOr))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsLogicalOrAssignment;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_logical_or_assignment:{
      setup: OperatorsLogicalOrAssignment::default(),
      source_code: r#"
        let a = null;
        let b = 3;
        a ||= 5;
      "#,
      eq: [
        r#"a ||= 5"#,
      ],
      ne: []
    }
  }
}
