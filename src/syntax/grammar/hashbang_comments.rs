use std::sync::OnceLock;

use oxc_ast::ast::Program;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_program(ctx: &mut Context, it: &Program) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./hashbang_comments.json")).unwrap()
  });
  if it.hashbang.is_some() {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_hashbang_comments(v: &mut SyntaxVisitor) {
  v.walk_program.push(walk_program);
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  assert_ok_count! {
    "hashbang_comments",
    crate::syntax::grammar::hashbang_comments::setup_hashbang_comments,

    should_ok_when_use_hashbang_comments,
    r#"#!/usr/bin/env node
  console.log("Hello world");"#,
    1,

    should_fail_when_hashbang_comments,
    r#"
      console.log("Hello world");
    "#,
    0,
  }
}
