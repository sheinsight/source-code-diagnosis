use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_empty_statement.push(walk_empty_statement);
  },
  compat {
    name: "empty",
    description: "空语句 (;)",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "3",
      chrome_android: "3",
      firefox: "1",
      firefox_android: "1",
      safari: "5",
      safari_ios: "5",
      edge: "12",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_empty_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::EmptyStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "empty",
    setup,
    should_ok_when_empty_statement,
    r#"
    ;
    "#,
    1,
  }
}
