use oxc_ast::{
  ast::{AssignmentTarget, BindingPatternKind},
  AstKind,
};

use crate::create_compat;

create_compat! {
  Destructuring,
  compat {
    name: "operators.destructuring",
    description: "解构赋值",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Destructuring_assignment",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "41",
      firefox_android: "41",
      safari: "8",
      safari_ios: "8",
      edge: "14",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::VariableDeclarator(variable_declarator) = node.kind() {
        return match variable_declarator.id.kind {
          BindingPatternKind::ObjectPattern(_) |
          BindingPatternKind::AssignmentPattern(_)|
          BindingPatternKind::ArrayPattern(_) => true,
          _ => false,
        }
    }

    if let AstKind::AssignmentExpression(assignment_expression) = node.kind() {
        return match assignment_expression.left {
            AssignmentTarget::ArrayAssignmentTarget(_)|
            AssignmentTarget::ObjectAssignmentTarget(_) => true,
            _ => false,
          }
    }

    false
  }
}

#[cfg(test)]
mod tests {
  use super::Destructuring;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_destructuring_of_object:{
      setup: Destructuring::default(),
      source_code: r#"
        const {a, b} = object;
      "#,
      eq: [
        r#"{a, b} = object"#,
      ],
      ne: []
    },

    should_ok_when_use_destructuring_of_array:{
      setup: Destructuring::default(),
      source_code: r#"
        const [a, b] = array;
      "#,
      eq: [
        r#"[a, b] = array"#,
      ],
      ne: []
    },

    should_ok_when_use_destructuring_of_assignment:{
      setup: Destructuring::default(),
      source_code: r#"
        let a, b;
        [a, b] = array;
      "#,
      eq: [
        r#"[a, b] = array"#,
      ],
      ne: []
    },

    should_ok_when_use_destructuring_of_computed_property_names:{
      setup: Destructuring::default(),
      source_code: r#"
        const key = "z";
        const { [key]: a } = obj;
      "#,
      eq: [
        r#"{ [key]: a } = obj"#,
      ],
      ne: []
    },

    should_ok_when_use_destructuring_of_rest_in_arrays:{
      setup: Destructuring::default(),
      source_code: r#"
        const [a, ...b] = array;
      "#,
      eq: [
        r#"[a, ...b] = array"#,
      ],
      ne: []
    },

    should_ok_when_use_destructuring_of_rest_in_objects:{
      setup: Destructuring::default(),
      source_code: r#"
        const {a, ...b} = object;
      "#,
      eq: [
        r#"{a, ...b} = object"#,
      ],
      ne: []
    },

    should_ok_when_use_destructuring_of_for_of:{
      setup: Destructuring::default(),
      source_code: r#"
        const people = [];
        for (const {
          name: n,
          family: { father: f },
        } of people) {
          console.log(`Name: ${n}, Father: ${f}`);
        }
      "#,
      eq: [
        r#"{
          name: n,
          family: { father: f },
        }"#,
      ],
      ne: []
    }
  }
}
