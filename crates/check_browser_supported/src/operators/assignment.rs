use oxc::ast::AstKind;
use oxc::syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  Assignment,
  compat {
    name: "operators.assignment",
    description: "赋值运算符 (<code>x = y</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Assignment",
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
    match node.kind() {
      AstKind::VariableDeclarator(declarator) => declarator.init.is_some(),
      AstKind::AssignmentExpression(assignment) => matches!(assignment.operator, AssignmentOperator::Assign),
      _ => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Assignment;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_basic_assignment: {
      setup: Assignment::default(),
      source_code: r#"
        let x = 5;
      "#,
      eq: [
        r#"x = 5"#,
      ],
      ne: []
    },

    should_ok_when_continuous_assignment: {
      setup: Assignment::default(),
      source_code: r#"
        let a, b, c;
        a = b = c = 2;
      "#,
      eq: [
        r#"a = b = c = 2"#,
        r#"b = c = 2"#,
        r#"c = 2"#,
      ],
      ne: []
    },

    should_ok_when_deconstruct_assignment: {
      setup: Assignment::default(),
      source_code: r#"
        let [d, e] = [1, 2];
      "#,
      eq: [
        r#"[d, e] = [1, 2]"#,
      ],
      ne: []
    },

    should_ok_when_object_deconstruct_assignment: {
      setup: Assignment::default(),
      source_code: r#"
        let {f, g} = {f: 3, g: 4};
      "#,
      eq: [
        r#"{f, g} = {f: 3, g: 4}"#,
      ],
      ne: []
    },
  }
}
