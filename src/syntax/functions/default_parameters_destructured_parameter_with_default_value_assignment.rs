use oxc_ast::{
  ast::{BindingPattern, BindingPatternKind},
  Visit,
};

use crate::create_compat;

fn has_assignment_pattern(binding_pattern: &BindingPattern) -> bool {
  struct Checker {
    found: bool,
  }
  impl<'a> Visit<'a> for Checker {
    fn visit_assignment_pattern(
      &mut self,
      _pattern: &oxc_ast::ast::AssignmentPattern<'a>,
    ) {
      self.found = true;
    }
  }
  let mut checker = Checker { found: false };
  checker.visit_binding_pattern(binding_pattern);
  checker.found
}

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_formal_parameter.push(default_parameters_destructured_parameter_with_default_value_assignment);
  },
  compat {
      name: "default_parameters_destructured_parameter_with_default_value_assignment",
      description: "destructured parameter with default value assignment",
      tags: [
          "web-features:default-parameters-destructured-parameter-with-default-value-assignment",
          "web-features:snapshot:ecmascript-2015"
      ],
      support: {
          chrome: "49",
          chrome_android: "49",
          firefox: "41",
          firefox_android: "41",
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
  default_parameters_destructured_parameter_with_default_value_assignment,
  |ctx: &mut Context, it: &oxc_ast::ast::FormalParameter| {
      if let BindingPatternKind::AssignmentPattern(pattern) = &it.pattern.kind {
          has_assignment_pattern(&pattern.left)
      } else {
          false
      }
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
      "default_parameters_destructured_parameter_with_default_value_assignment",
      setup,

      should_ok_when_use_default_parameters_destructured_parameter_with_default_value_assignment,
      r#"
        function f({ a = 1 } = {}) {}
      "#,
      1,

      should_ok_when_not_use_default_parameters_destructured_parameter_with_default_value_assignment,
      r#"
        function f(a) {}
      "#,
      0,
  }
}
