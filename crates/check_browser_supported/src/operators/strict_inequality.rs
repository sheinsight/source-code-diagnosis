use oxc_ast::AstKind;
use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  OperatorsStrictInequality,
  compat {
    name: "operators.strict_inequality",
    description: "严格不等运算符 (<code>a !== b</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Strict_inequality",
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
    matches!(node.kind(), AstKind::BinaryExpression(expr) if expr.operator == BinaryOperator::StrictInequality)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsStrictInequality;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_strict_inequality:{
      setup: OperatorsStrictInequality::default(),
      source_code: r#"
        console.log(5 !== 5);
        console.log("hello" !== "world");
      "#,
      eq: [
        r#"5 !== 5"#,
        r#""hello" !== "world""#,
      ],
      ne: []
    }
  }
}
