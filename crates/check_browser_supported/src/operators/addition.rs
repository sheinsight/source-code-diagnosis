use oxc_ast::AstKind;
use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  Addition,
  compat {
    name: "operators.addition",
    description: "加法运算符 (+)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Addition",
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
    if let AstKind::BinaryExpression(binary_expr) = node.kind() {
      matches!(binary_expr.operator, BinaryOperator::Addition)
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Addition;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_addition:{
      setup: Addition::default(),
      source_code: r#"
        console.log(2 + 2);
        let x = 5 + 3;
      "#,
      eq: [
        r#"2 + 2"#,
        r#"5 + 3"#,
      ],
      ne: [
        r#"console.log"#,
        r#"let x ="#,
      ]
    }
  }
}
