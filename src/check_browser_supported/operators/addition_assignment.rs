use oxc_ast::AstKind;
use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat_2;

create_compat_2! {
  AdditionAssignment,
  compat {
    name: "operators.addition_assignment",
    description: "加法赋值运算符 (x += y)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Addition_assignment",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
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
    if let AstKind::AssignmentExpression(assignment) = node.kind() {
      matches!(assignment.operator, AssignmentOperator::Addition)
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::AdditionAssignment;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_addition_assignment:{
      setup: AdditionAssignment::default(),
      source_code: r#"
        let a = 2;
        let b = 'hello';
        console.log((a += 3));
      "#,
      eq: [
        r#"a += 3"#,
      ],
      ne: []
    },

    should_ok_when_use_addition_assignment_with_string:{
      setup: AdditionAssignment::default(),
      source_code: r#"
        let a = 'hello';
        let b = 'world';
        console.log((a += b));
      "#,
      eq: [
        r#"a += b"#,
      ],
      ne: []
    }
  }
}
