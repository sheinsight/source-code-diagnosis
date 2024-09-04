use oxc_syntax::operator::UnaryOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsUnaryPlus,
  compat {
    name: "operators_unary_plus",
    description: "一元加法运算符 (+)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Unary_plus",
    tags: ["web-features:snapshot:ecmascript-1"],
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
    matches!(node.kind(), AstKind::UnaryExpression(expr) if matches!(expr.operator, UnaryOperator::UnaryPlus))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsUnaryPlus;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_unary_plus:{
      setup: OperatorsUnaryPlus::default(),
      source_code: r#"
        let x = 5;
        console.log(+x);
        let y = +"42";
      "#,
      eq: [
        r#"+x"#,
        r#"+"42""#,
      ],
      ne: []
    }
  }
}
