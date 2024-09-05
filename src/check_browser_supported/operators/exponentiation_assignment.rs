use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat_2;

create_compat_2! {
  ExponentiationAssignment,
  compat {
    name: "operators.exponentiation_assignment",
    description: "幂赋值运算符 (<code>x **= y</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Exponentiation_assignment",
    tags: [
      "web-features:snapshot:ecmascript-2016"
    ],
    support: {
      chrome: "52",
      chrome_android: "52",
      firefox: "52",
      firefox_android: "52",
      safari: "10",
      safari_ios: "10",
      edge: "14",
      node: "7.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::AssignmentExpression(expr) if matches!(expr.operator, AssignmentOperator::Exponential))
  }
}

#[cfg(test)]
mod tests {
  use super::ExponentiationAssignment;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_exponentiation_assignment:{
      setup: ExponentiationAssignment::default(),
      source_code: r#"
        let x = 2;
        x **= 3;
      "#,
      eq: [
        r#"x **= 3"#,
      ],
      ne: []
    }
  }
}
