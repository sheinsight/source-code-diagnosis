use oxc_ast::AstKind;
use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  OperatorsLessThanOrEqual,
  compat {
    name: "operators.less_than_or_equal",
    description: "小于等于运算符 (<code>a &lt;= b</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Less_than_or_equal",
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
    matches!(node.kind(), AstKind::BinaryExpression(expr) if expr.operator == BinaryOperator::LessEqualThan)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsLessThanOrEqual;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_less_than_or_equal:{
      setup: OperatorsLessThanOrEqual::default(),
      source_code: r#"
        console.log(5 <= 3);
        console.log(3 <= 3);
      "#,
      eq: [
        r#"5 <= 3"#,
        r#"3 <= 3"#,
      ],
      ne: []
    }
  }
}
