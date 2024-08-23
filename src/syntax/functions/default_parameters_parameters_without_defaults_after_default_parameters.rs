use oxc_ast::ast::BindingPatternKind;

use crate::create_compat;

create_compat! {
    setup,
    |v: &mut SyntaxVisitor| {
        v.walk_formal_parameters.push(default_parameters_parameters_without_defaults_after_default_parameters);
    },
    compat {
        name: "default_parameters_parameters_without_defaults_after_default_parameters",
        description: "parameters without defaults after default parameters",
        tags: [
            "web-features:default-parameters-parameters-without-defaults-after-default-parameters",
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
    default_parameters_parameters_without_defaults_after_default_parameters,
    |ctx: &mut Context, it: &oxc_ast::ast::FormalParameters| {
      let mut flag: i32 = 0;
      for item in &it.items {
        match item.pattern.kind {
          BindingPatternKind::AssignmentPattern(_) => {
            flag = 1;
          }
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
      flag == -1
    }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
      "default_parameters_parameters_without_defaults_after_default_parameters",
      setup,

      should_ok_when_use_default_parameters_parameters_without_defaults_after_default_parameters,
      r#"
          function example(a = 1, b) {}
        "#,
      1,

      should_ok_when_not_use_default_parameters_parameters_without_defaults_after_default_parameters,
      r#"
          function example(a, b) {}
        "#,
      0,
  }
}
