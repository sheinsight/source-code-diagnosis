use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat_2;

create_compat_2! {
  LogicalAndAssignment,
  compat {
    name: "operators.logical_and_assignment",
    description: "逻辑与赋值运算符 (x &&= y)",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Logical_AND_assignment",
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
    matches!(node.kind(), AstKind::AssignmentExpression(expr) if matches!(expr.operator, AssignmentOperator::LogicalAnd))
  }
}

#[cfg(test)]
mod tests {
  use super::LogicalAndAssignment;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_logical_and_assignment:{
      setup: LogicalAndAssignment::default(),
      source_code: r#"
        let a = 1;
        let b = 0;
        a &&= 2;
      "#,
      eq: [
        r#"a &&= 2"#,
      ],
      ne: []
    }
  }
}
