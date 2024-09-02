use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_program.push(walk_program);
  },
  compat {
    name: "hashbang_comments",
    description: "Hashbang (#!) 注释语法",
    tags: [
      "web-features:snapshot:ecmascript-2023"
    ],
    support: {
      chrome: "74",
      chrome_android: "74",
      firefox: "67",
      firefox_android: "67",
      safari: "13.1",
      safari_ios: "13.1",
      edge: "74",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_program,
  |ctx: &mut Context, it: &oxc_ast::ast::Program| {
    it.hashbang.is_some()
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "hashbang_comments",
    setup,

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
