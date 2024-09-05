use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat_2;

create_compat_2! {
  SubtractionAssignment,
  compat {
    name: "operators.subtraction_assignment",
    description: "减法赋值运算符 (<code>x -= y</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Subtraction_assignment",
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
    matches!(node.kind(), AstKind::AssignmentExpression(expr) if matches!(expr.operator, AssignmentOperator::Subtraction))
  }
}

#[cfg(test)]
mod tests {
  use super::SubtractionAssignment;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_subtraction_assignment:{
      setup: SubtractionAssignment::default(),
      source_code: r#"
        let x = 10;
        x -= 5;
        console.log(x);
      "#,
      eq: [
        r#"x -= 5"#,
      ],
      ne: []
    }
  }
}
