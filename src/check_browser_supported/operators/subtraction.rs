use oxc_ast::AstKind;
use oxc_syntax::operator::BinaryOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsSubtraction,
  compat {
    name: "operators.subtraction",
    description: "减法运算符 (<code>-</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Subtraction",
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
    matches!(node.kind(), AstKind::BinaryExpression(expr) if expr.operator == BinaryOperator::Subtraction)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsSubtraction;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_subtraction:{
      setup: OperatorsSubtraction::default(),
      source_code: r#"
        console.log(5 - 3);
        let x = 10 - 7;
      "#,
      eq: [
        r#"5 - 3"#,
        r#"10 - 7"#,
      ],
      ne: []
    }
  }
}
