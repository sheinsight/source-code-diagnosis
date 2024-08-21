use std::sync::OnceLock;

use oxc_ast::{
  ast::{BindingPattern, BindingPatternKind},
  Visit,
};

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

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

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_formal_parameter(
  ctx: &mut Context,
  it: &oxc_ast::ast::FormalParameter,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./default_parameters_destructured_parameter_with_default_value_assignment.json")).unwrap()
  });

  if let BindingPatternKind::AssignmentPattern(pattern) = &it.pattern.kind {
    if has_assignment_pattern(&pattern.left) {
      ctx.usage.push(CompatBox::new(it.span, compat.clone()));
    }
  }
}

pub fn setup_default_parameters_destructured_parameter_with_default_value_assignment(
  v: &mut SyntaxVisitor,
) {
  v.walk_formal_parameter.push(walk_formal_parameter);
}

#[cfg(test)]
mod tests {
  use super::setup_default_parameters_destructured_parameter_with_default_value_assignment;
  use crate::assert_ok_count;

  assert_ok_count! {
    "functions_default_parameters_destructured_parameter_with_default_value_assignment",
    setup_default_parameters_destructured_parameter_with_default_value_assignment,

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
