use std::sync::OnceLock;

use oxc_span::Span;
use regex::Regex;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
};

fn get_source_code(source_code: &str, span: Span) -> &str {
  &source_code[span.start as usize..span.end as usize]
}

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_array_expression(
  ctx: &mut Context,
  it: &oxc_ast::ast::ArrayExpression,
) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./trailing_commas.json")).unwrap());
  let source_code = get_source_code(&ctx.source_code.as_str(), it.span);
  if let Ok(regex) = Regex::new(r",\s*\]$") {
    if regex.is_match(source_code) {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

fn walk_object_expression(
  ctx: &mut Context,
  it: &oxc_ast::ast::ObjectExpression,
) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./trailing_commas.json")).unwrap());
  let source_code = get_source_code(&ctx.source_code.as_str(), it.span);
  if let Ok(regex) = Regex::new(r",\s*\}$") {
    if regex.is_match(source_code) {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

fn walk_function(
  ctx: &mut Context,
  it: &oxc_ast::ast::Function,
  flags: &oxc_semantic::ScopeFlags,
  is_strict_mode: bool,
) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./trailing_commas.json")).unwrap());
  let source_code = get_source_code(&ctx.source_code.as_str(), it.params.span);
  if let Ok(regex) = Regex::new(r",\s*\)$") {
    if regex.is_match(source_code) {
      ctx
        .usage
        .push(CompatBox::new(it.params.span.clone(), compat.clone()));
    }
  }
}

fn walk_call_expression(ctx: &mut Context, it: &oxc_ast::ast::CallExpression) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./trailing_commas.json")).unwrap());
  let source_code = get_source_code(&ctx.source_code.as_str(), it.span);
  if let Ok(regex) = Regex::new(r",\s*\)$") {
    if regex.is_match(source_code) {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

fn walk_import_declaration(
  ctx: &mut Context,
  it: &oxc_ast::ast::ImportDeclaration,
) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./trailing_commas.json")).unwrap());
  let source_code = get_source_code(&ctx.source_code.as_str(), it.span);
  if let Ok(regex) =
    Regex::new(r##"\{\s*[^}]*,\s*}\s*from\s*['\"][^'\"]*['\"];?"##)
  {
    if regex.is_match(source_code) {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

fn walk_export_named_declaration(
  ctx: &mut Context,
  it: &oxc_ast::ast::ExportNamedDeclaration,
) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./trailing_commas.json")).unwrap());
  let source_code = get_source_code(&ctx.source_code.as_str(), it.span);
  if let Ok(regex) = Regex::new(r"export\s*\{\s*[^}]*,\s*}") {
    if regex.is_match(source_code) {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_trailing_commas(v: &mut crate::syntax::visitor::SyntaxVisitor) {
  v.walk_array_expression.push(walk_array_expression);
  v.walk_object_expression.push(walk_object_expression);
  v.walk_function.push(walk_function);
  v.walk_call_expression.push(walk_call_expression);
  v.walk_import_declaration.push(walk_import_declaration);
  v.walk_export_named_declaration
    .push(walk_export_named_declaration);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count, syntax::grammar::trailing_commas::setup_trailing_commas,
  };

  assert_ok_count! {
    "trailing_commas",
    setup_trailing_commas,

    should_ok_when_array_expression,
    r#"
      const arr = [
        1,
        2,
        3,
      ];
    "#,
    1,

    should_ok_when_object_literals,
    r#"
      const object = {
        foo: "bar",
        baz: "qwerty",
        age: 42,
      };
    "#,
    1,

    should_ok_when_function,
    r#"
      function f(p,) {}
    "#,
    1,

    should_ok_when_class_function,
    r#"
      class C {
        one(a,) {}
        two(a, b,) {}
      }
    "#,
    2,

    should_ok_when_object_function,
    r#"
      const obj = {
        one(a,) {},
        two(a, b,) {}
      };
    "#,
    2,

    should_ok_when_function_call_expression,
    r#"
      hello(a,b,)
    "#,
    1,

    should_ok_when_named_import,
    r#"
      import {
        A,
        B,
        C,
      } from "D";
    "#,
    1,

    should_ok_when_named_exported,
    r#"
      export {
        A,
        B,
        C,
      };

      export { A, B, C, };

      export { A as B, C as D, E as F, };
    "#,
    3,

  }
}
