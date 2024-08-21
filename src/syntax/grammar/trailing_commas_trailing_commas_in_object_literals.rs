use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_object_expression(
  ctx: &mut Context,
  it: &oxc_ast::ast::ObjectExpression,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!(
      "./trailing_commas_trailing_commas_in_object_literals.json"
    ))
    .unwrap()
  });
  if it.trailing_comma.is_some() {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_trailing_commas_trailing_commas_in_object_literals(
  v: &mut crate::syntax::visitor::SyntaxVisitor,
) {
  v.walk_object_expression.push(walk_object_expression);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::trailing_commas_trailing_commas_in_object_literals::setup_trailing_commas_trailing_commas_in_object_literals,
  };

  assert_ok_count! {
    "trailing_commas_trailing_commas_in_object_literals",
    setup_trailing_commas_trailing_commas_in_object_literals,

    should_ok_when_object_expression,
    r#"
      const obj = {
        prop1: 'value1',
        prop2: 'value2',
        prop3: 'value3',
      };
    "#,
    1,

    should_ok_when_object_expression_then,
    r#"
      const obj = {
        prop1: 'value1',
        prop2: 'value2',
        prop3: 'value3',
      };

      const obj2 = {
        prop1: 'value1',
        prop2: 'value2',
        prop3: 'value3',
      };
    "#,
    2,
  }
}
