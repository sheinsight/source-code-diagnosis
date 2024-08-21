use std::sync::OnceLock;

use oxc_ast::ast::Expression;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_static_member_expression(
  ctx: &mut Context,
  it: &oxc_ast::ast::StaticMemberExpression,
) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./arguments_length.json")).unwrap());

  if let Expression::Identifier(o) = &it.object {
    if o.name == "arguments" && it.property.name == "length" {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_arguments_length(v: &mut SyntaxVisitor) {
  v.walk_static_member_expression
    .push(walk_static_member_expression);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::functions::arguments_length::setup_arguments_length,
  };

  assert_ok_count! {
    "functions_arguments_length",
    setup_arguments_length,

    should_ok_when_use_arguments_length,
    r#"
      function logArguments() {
          console.log(` ${arguments.length} `);
          for (let i = 0; i < arguments.length; i++) {
              console.log(`${i + 1}: ${arguments[i]}`);
          }
      }
    "#,
    2,

    should_ok_when_not_use_arguments_length,
    r#"
      function logArguments() {
          
      }
    "#,
    0,

    should_ok_when_use_arguments_length_in_arrow_function,
    r#"
      const logArguments = () => {
          console.log(` ${arguments.length} `);
          for (let i = 0; i < arguments.length; i++) {
              console.log(`${i + 1}: ${arguments[i]}`);
          }
      }
    "#,
    2,

  }
}
