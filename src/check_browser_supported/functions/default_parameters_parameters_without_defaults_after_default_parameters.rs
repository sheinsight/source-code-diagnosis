use oxc_ast::{ast::BindingPatternKind, AstKind};

use crate::create_compat_2;

create_compat_2! {
  DefaultParametersParametersWithoutDefaultsAfterDefaultParameters,
  compat {
    name: "default_parameters.parameters_without_defaults_after_default_parameters",
    description: "destructured parameter with default value assignment",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/Default_parameters#destructured_parameter_with_default_value_assignment",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "26",
      firefox_android: "26",
      safari: "10",
      safari_ios: "10",
      edge: "14",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::FormalParameters(params) = node.kind() {
      let mut flag: i32 = 0;
      for item in &params.items {
        match item.pattern.kind {
          BindingPatternKind::AssignmentPattern(_) => {
            flag = 1;
          },
          BindingPatternKind::BindingIdentifier(_)
          | BindingPatternKind::ObjectPattern(_)
          | BindingPatternKind::ArrayPattern(_) => {
            if flag == 1 {
              flag = -1;
            } else {
              flag = 0;
            }
          }
        }
      }
      return flag == -1;
    }

    false
  }
}

#[cfg(test)]
mod tests {

  use super::DefaultParametersParametersWithoutDefaultsAfterDefaultParameters;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: DefaultParametersParametersWithoutDefaultsAfterDefaultParameters::default(),
      source_code: r#"
         function example(a = 1, b) {}
      "#,
      eq: [
        r#"(a = 1, b)"#,
      ],
      ne: [ ]
    },

    should_ok_when_use_class_declaration2:{
      setup: DefaultParametersParametersWithoutDefaultsAfterDefaultParameters::default(),
      source_code: r#"
         function f(bb,{ a = 1 } = {}) {}
      "#,
      eq: [ ],
      ne: [ ]
    },
  }
}
