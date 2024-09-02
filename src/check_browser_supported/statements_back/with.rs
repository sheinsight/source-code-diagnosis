use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_with_statement.push(walk_with_statement);
  },
  compat {
    name: "with",
    description: "with 语句",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_with_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::WithStatement| {
    true
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "with",
    setup,

    should_ok_when_use_with_statement,
    r#"
    with ([1, 2, 3]) {
      console.log(toString()); // 1,2,3
    }    
    "#,
    1
  }
}
