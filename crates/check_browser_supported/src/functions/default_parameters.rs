use oxc_ast::{ast::BindingPatternKind, AstKind};

use crate::create_compat;

create_compat! {
  DefaultParameters,
  compat {
    name: "default_parameters",
    description: "default function parameters",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Default_parameters",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "15",
      firefox_android: "15",
      safari: "10",
      safari_ios: "10",
      edge: "14",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    matches!(
      node.kind(), AstKind::FormalParameter(param)
      if matches!(param.pattern.kind, BindingPatternKind::AssignmentPattern(_))
    )

  }
}

#[cfg(test)]
mod tests {

  use super::DefaultParameters;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: DefaultParameters::default(),
      source_code: r#"
         function multiply(a, b = 1) {
          return a * b;
        }
      "#,
      eq: [
        r#"b = 1"#,
      ],
      ne: [ ]
    },

    should_ok_when_use_class_declaration2:{
      setup: DefaultParameters::default(),
      source_code: r#"
         function multiply(a = 1, b = 1) {
          return a * b;
        }
      "#,
      eq: [
        r#"a = 1"#,
        r#"b = 1"#,
      ],
      ne: [ ]
    },
  }
}
