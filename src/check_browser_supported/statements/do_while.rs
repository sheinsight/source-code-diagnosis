use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_do_while_statement.push(walk_do_while_statement);
  },
  compat {
    name: "do_while",
    description: "do...while",
    tags: ["web-features:snapshot:ecmascript-3"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "4",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_do_while_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::DoWhileStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "do_while",
    setup,
    should_ok_when_do_while_statement,
    r#"
    do {
      i = i + 1;
      result = result + i;
    } while (i < 5);
    "#,
    1,
  }
}
