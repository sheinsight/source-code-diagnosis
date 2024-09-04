use oxc_syntax::operator::UnaryOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsLogicalNot,
  compat {
    name: "operators.logical_not",
    description: "逻辑非运算符 (<code>!</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Logical_NOT",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
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
    matches!(node.kind(), AstKind::UnaryExpression(expr) if expr.operator == UnaryOperator::LogicalNot)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsLogicalNot;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_logical_not:{
      setup: OperatorsLogicalNot::default(),
      source_code: r#"
        console.log(!true);
        let x = !false;
      "#,
      eq: [
        r#"!true"#,
        r#"!false"#,
      ],
      ne: []
    }
  }
}
