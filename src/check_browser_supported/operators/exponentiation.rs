use oxc_syntax::operator::BinaryOperator;

use crate::create_compat_2;

create_compat_2! {
  Exponentiation,
  compat {
    name: "operators.exponentiation",
    description: "幂运算符 (**)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Exponentiation",
    tags: ["web-features:snapshot:ecmascript-2016"],
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
    matches!(node.kind(), AstKind::BinaryExpression(expr) if matches!(expr.operator, BinaryOperator::Exponential))
  }
}

#[cfg(test)]
mod tests {
  use super::Exponentiation;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_exponentiation:{
      setup: Exponentiation::default(),
      source_code: r#"
        console.log(2 ** 3);
      "#,
      eq: [
        r#"2 ** 3"#,
      ],
      ne: []
    }
  }
}
