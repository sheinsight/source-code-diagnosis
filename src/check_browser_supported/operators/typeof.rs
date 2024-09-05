use oxc_syntax::operator::UnaryOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsTypeof,
  compat {
    name: "operators.typeof",
    description: "typeof 运算符返回一个字符串，表示未经计算的操作数的类型。",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/typeof",
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
    matches!(node.kind(), AstKind::UnaryExpression(unary_expr) if unary_expr.operator == UnaryOperator::Typeof)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsTypeof;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_typeof_number:{
      setup: OperatorsTypeof::default(),
      source_code: r#"
        typeof 42;
        typeof "hello";
        typeof true;
        typeof undefined;
        typeof { a: 1 };
        typeof [1, 2, 3];
        typeof function() {};
      "#,
      eq: [
        r#"typeof 42"#,
        r#"typeof "hello""#,
        r#"typeof true"#,
        r#"typeof undefined"#,
        r#"typeof { a: 1 }"#,
        r#"typeof [1, 2, 3]"#,
        r#"typeof function() {}"#,
      ],
      ne: []
    }
  }
}
