use oxc_syntax::operator::BinaryOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsEquality,
  compat {
    name: "operators.equality",
    description: "相等运算符 (==)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Equality",
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
    matches!(node.kind(), AstKind::BinaryExpression(expr) if expr.operator == BinaryOperator::Equality)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsEquality;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_equality:{
      setup: OperatorsEquality::default(),
      source_code: r#"
        console.log(1 == 1);
        console.log("a" == "b");
      "#,
      eq: [
        r#"1 == 1"#,
        r#""a" == "b""#,
      ],
      ne: []
    }
  }
}
