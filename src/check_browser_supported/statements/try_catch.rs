use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_try_statement.push(walk_try_statement);
  },
  compat {
    name: "try_catch",
    description: "try...catch 语句",
    tags: ["web-features:snapshot:ecmascript-3"],
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
  walk_try_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::TryStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "try_catch",
    setup,

    should_ok_when_use_try_expression,
    r#"
    try {
      nonExistentFunction();
    } catch (error) {
      console.error(error);
    }
    "#,
    1
  }
}
