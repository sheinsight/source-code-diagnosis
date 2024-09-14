use oxc_ast::AstKind;
use oxc_syntax::operator::UnaryOperator;

use crate::create_compat;

create_compat! {
  OperatorsVoid,
  compat {
    name: "operators.void",
    description: "void 运算符对给定的表达式进行求值，然后返回 undefined。",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/void",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "3.1",
      safari_ios: "3",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::UnaryExpression(unary_expr) if unary_expr.operator == UnaryOperator::Void)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsVoid;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_void_operator:{
      setup: OperatorsVoid::default(),
      source_code: r#"
        console.log(void 0);
        let result = void (2 + 3);
      "#,
      eq: [
        r#"void 0"#,
        r#"void (2 + 3)"#,
      ],
      ne: [
        r#"console.log"#,
      ]
    }
  }
}
