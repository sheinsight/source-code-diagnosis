use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_throw_statement.push(walk_throw_statement);
  },
  compat {
    name: "throw",
    description: "throw 语句",
    tags: [
      "web-features:snapshot:ecmascript-3"
    ],
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
  walk_throw_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ThrowStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "throw",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    throw new Error('Parameter is not a number!');
    "#,
    1,
  }
}
