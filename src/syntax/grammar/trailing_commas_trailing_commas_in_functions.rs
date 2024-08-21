use std::sync::OnceLock;

use regex::Regex;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_function(
  ctx: &mut Context,
  it: &oxc_ast::ast::Function,
  _flags: &oxc_semantic::ScopeFlags,
  _is_strict_mode: bool,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!(
      "./trailing_commas_trailing_commas_in_functions.json"
    ))
    .unwrap()
  });
  let source_code = &ctx.source_code
    [it.params.span.start as usize..it.params.span.end as usize];
  if let Ok(regex) = Regex::new(r",\s*\)$") {
    if regex.is_match(source_code) {
      ctx
        .usage
        .push(CompatBox::new(it.params.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_trailing_commas_trailing_commas_in_functions(
  v: &mut crate::syntax::visitor::SyntaxVisitor,
) {
  v.walk_function.push(walk_function);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::trailing_commas_trailing_commas_in_functions::setup_trailing_commas_trailing_commas_in_functions,
  };

  assert_ok_count! {
    "trailing_commas_trailing_commas_in_functions",
    setup_trailing_commas_trailing_commas_in_functions,

    should_ok_when_function_declaration,
    r#"
      function myFunction(
        param1,
        param2,
      ) {

      }
    "#,
    1,

    should_ok_when_function_declaration_then,
    r#"
      function myFunction(
        param1,
        param2,
      ) {

      }

      function myFunction2(
        param1,
        param2,
      ) {

      }
    "#,
    2,
  }
}
