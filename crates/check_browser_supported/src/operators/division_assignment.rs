use oxc::ast::AstKind;
use oxc::syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  DivisionAssignment,
  compat {
    name: "operators.division_assignment",
    description: "除法赋值运算符 (<code>x /= y</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Division_assignment",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "18",
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
    matches!(node.kind(), AstKind::AssignmentExpression(expr) if matches!(expr.operator, AssignmentOperator::Division))
  }
}

#[cfg(test)]
mod tests {
  use super::DivisionAssignment;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_division_assignment:{
      setup: DivisionAssignment::default(),
      source_code: r#"
        let a = 10;
        a /= 2;
        console.log(a);
      "#,
      eq: [
        r#"a /= 2"#,
      ],
      ne: []
    }
  }
}
