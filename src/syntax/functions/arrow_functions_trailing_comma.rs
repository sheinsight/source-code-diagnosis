use std::sync::OnceLock;

use oxc_span::Span;
use regex::Regex;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn get_source_code(source_code: &str, span: Span) -> &str {
  &source_code[span.start as usize..span.end as usize]
}

pub fn walk_arrow_function_expression(
  ctx: &mut Context,
  it: &oxc_ast::ast::ArrowFunctionExpression,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./arrow_functions_trailing_comma.json")).unwrap()
  });

  if let Ok(regex) = Regex::new(r",\s*\)$") {
    let params_source_code = get_source_code(&ctx.source_code, it.params.span);
    if regex.is_match(params_source_code) {
      ctx
        .usage
        .push(CompatBox::new(it.params.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_arrow_functions_trailing_comma(v: &mut SyntaxVisitor) {
  v.walk_arrow_function_expression
    .push(walk_arrow_function_expression);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::functions::arrow_functions_trailing_comma::setup_arrow_functions_trailing_comma,
  };

  assert_ok_count! {
    "functions_arrow_functions_trailing_comma",
    setup_arrow_functions_trailing_comma,

    should_ok_when_use_arrow_functions_trailing_comma,
    r#"
      const func4 = (
        a,
        b,
        c,
      ) => 1;
    "#,
    1,

    should_ok_when_not_use_arrow_functions_trailing_comma,
    r#"
      const func4 = (
        a,
        b,
        c
      ) => 1;
    "#,
    0,

    should_ok_when_use_arrow_functions_trailing_comma_with_no_space,
    r#"
      const func4 = (
        a,
        b,
        c,) => 1;
    "#,
    1,
  }
}
