use std::sync::OnceLock;

use oxc_ast::ast::BindingPatternKind;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_formal_parameters(
  ctx: &mut Context,
  it: &oxc_ast::ast::FormalParameters,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./default_parameters_parameters_without_defaults_after_default_parameters.json")).unwrap()
  });

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

    if flag == -1 {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_default_parameters_parameters_without_defaults_after_default_parameters(
  v: &mut SyntaxVisitor,
) {
  v.walk_formal_parameters.push(walk_formal_parameters);
}

#[cfg(test)]
mod tests {
  use super::setup_default_parameters_parameters_without_defaults_after_default_parameters;
  use crate::assert_ok_count;

  assert_ok_count! {
    "functions_default_parameters_parameters_without_defaults_after_default_parameters",
    setup_default_parameters_parameters_without_defaults_after_default_parameters,

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
