use oxc_ast::ast::BindingPatternKind;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_formal_parameter.push(walk_formal_parameter);
  },
  compat {
    name: "default_parameters",
    description: "default function parameters",
    tags: [
      "web-features:default-parameters",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "15",
      firefox_android: "15",
      opera: "36",
      opera_android: "36",
      safari: "10",
      safari_ios: "10",
      edge: "14",
      oculus: "5.0",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_formal_parameter,
  |ctx: &mut Context, it: &oxc_ast::ast::FormalParameter| {
    matches!(it.pattern.kind, BindingPatternKind::AssignmentPattern(_))
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "default_parameters",
    setup,

    should_ok_when_use_default_parameters,
    r#"
      function multiply(a, b = 1) {
        return a * b;
      }
    "#,
    1,

    should_ok_when_use_two_default_parameters,
    r#"
      function multiply(a = 1, b = 1) {
        return a * b;
      }
    "#,
    2,

    should_ok_when_not_use_default_parameters,
    r#"
      function multiply(a, b) {
        return a * b;
      }
    "#,
    0
  }
}
