use std::sync::OnceLock;

use regex::Regex;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_import_expression(
  ctx: &mut Context,
  it: &oxc_ast::ast::ImportExpression,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!(
      "./trailing_commas_trailing_commas_in_dynamic_import.json"
    ))
    .unwrap()
  });

  let source_code =
    &ctx.source_code[it.span.start as usize..it.span.end as usize];
  if let Ok(regex) = Regex::new(r",\s*\)$") {
    if regex.is_match(source_code) {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_trailing_commas_trailing_commas_in_dynamic_import(
  v: &mut crate::syntax::visitor::SyntaxVisitor,
) {
  v.walk_import_expression.push(walk_import_expression);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::trailing_commas_trailing_commas_in_dynamic_import::setup_trailing_commas_trailing_commas_in_dynamic_import,
  };

  assert_ok_count! {
    "trailing_commas_trailing_commas_in_dynamic_import",
    setup_trailing_commas_trailing_commas_in_dynamic_import,

    should_ok_when_import_expression,
    r#"
      import(
        './module.js',
      );
    "#,
    1,

    should_ok_when_import_expression_then,
    r#"
      import(
        './module.js',
      ).then(module => {

      });
    "#,
    1,
  }
}
