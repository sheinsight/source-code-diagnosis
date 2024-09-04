use oxc_syntax::operator::BinaryOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsIn,
  compat {
    name: "operators_in",
    description: "in 运算符",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/in",
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
    if let AstKind::BinaryExpression(binary_expr) = node.kind() {
      matches!(binary_expr.operator, BinaryOperator::In)
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsIn;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_in_operator:{
      setup: OperatorsIn::default(),
      source_code: r#"
        const car = { make: 'Honda', model: 'Accord', year: 1998 };
        console.log('make' in car);
        console.log('color' in car);
      "#,
      eq: [
        r#"'make' in car"#,
        r#"'color' in car"#,
      ],
      ne: []
    }
  }
}
