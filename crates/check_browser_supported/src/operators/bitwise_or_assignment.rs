use oxc_ast::AstKind;
use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  BitwiseOrAssignment,
  compat {
    name: "operators.bitwise_or_assignment",
    description: "按位或赋值运算符 (<code>x |= y</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Bitwise_OR_assignment",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "4",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::AssignmentExpression(expr) if matches!(expr.operator, AssignmentOperator::BitwiseOR))
  }
}

#[cfg(test)]
mod tests {
  use super::BitwiseOrAssignment;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_bitwise_or_assignment:{
      setup: BitwiseOrAssignment::default(),
      source_code: r#"
        let a = 5;
        a |= 3;
        console.log(a);
      "#,
      eq: [
        r#"a |= 3"#,
      ],
      ne: []
    }
  }
}
