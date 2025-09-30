use oxc::ast::AstKind;
use oxc::syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  OperatorsInstanceof,
  compat {
    name: "operators.instanceof",
    description: "instanceof 运算符",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/instanceof",
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
    matches!(node.kind(), AstKind::BinaryExpression(binary_expr) if binary_expr.operator == BinaryOperator::Instanceof)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsInstanceof;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_instanceof_operator:{
      setup: OperatorsInstanceof::default(),
      source_code: r#"
        console.log(auto instanceof Car);
      "#,
      eq: [
        r#"auto instanceof Car"#,
      ],
      ne: []
    }
  }
}
