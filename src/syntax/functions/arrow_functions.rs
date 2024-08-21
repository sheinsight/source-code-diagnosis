use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_arrow_function_expression(
  ctx: &mut Context,
  it: &oxc_ast::ast::ArrowFunctionExpression,
) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./arrow_functions.json")).unwrap());
  ctx
    .usage
    .push(CompatBox::new(it.span.clone(), compat.clone()));
}

pub fn setup_arrow_functions(v: &mut SyntaxVisitor) {
  v.walk_arrow_function_expression
    .push(walk_arrow_function_expression);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count, syntax::functions::arrow_functions::setup_arrow_functions,
  };

  assert_ok_count! {
    "functions_arrow_functions",
    setup_arrow_functions,

    should_ok_when_use_arrow_functions,
    r#"
      const materials = ['Hydrogen', 'Helium', 'Lithium', 'Beryllium'];

      console.log(materials.map((material) => material.length));
    "#,
    1,

    should_ok_when_not_use_arrow_functions,
    r#"
      const materials = ['Hydrogen', 'Helium', 'Lithium', 'Beryllium'];

      console.log(materials.map(function(material) { return material.length; }));
    "#,
    0,

    should_ok_when_use_arrow_functions_with_no_space,
    r#"
      const materials = ['Hydrogen', 'Helium', 'Lithium', 'Beryllium'];

      console.log(materials.map((material)=>material.length));
    "#,
    1,

    should_ok_when_use_arrow_functions_with_no_parentheses,
    r#"
      const materials = ['Hydrogen', 'Helium', 'Lithium', 'Beryllium'];

      console.log(materials.map(material => material.length));
    "#,
    1,
  }
}
