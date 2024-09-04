use oxc_syntax::operator::BinaryOperator;

use crate::create_compat_2;

create_compat_2! {
  Division,
  compat {
    name: "operators.division",
    description: "除法运算符 (<code>/</code>)",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Division",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1.0.0",
      chrome_android: "18.0.0",
      firefox: "1.0.0",
      firefox_android: "4.0.0",
      safari: "1.0.0",
      safari_ios: "1.0.0",
      edge: "12.0.0",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::BinaryExpression(binary_expr) = node.kind() {
      matches!(binary_expr.operator, BinaryOperator::Division)
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Division;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_division:{
      setup: Division::default(),
      source_code: r#"
        console.log(6 / 2);
      "#,
      eq: [
        r#"6 / 2"#,
      ],
      ne: []
    }
  }
}
