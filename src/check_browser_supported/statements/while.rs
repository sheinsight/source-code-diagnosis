use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_while_statement.push(walk_while_statement);
  },
  compat {
    name: "while",
    description: "while 循环",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "3",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_while_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::WhileStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "while",
    setup,
    should_ok_when_use_while_expression,
    r#"
    while (n < 3) {
      n++;
    }
    "#,
    1
  }
}
