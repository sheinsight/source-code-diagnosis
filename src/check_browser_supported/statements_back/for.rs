use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_for_statement.push(walk_for_statement);
  },
  compat {
    name: "for",
    description: "The `for` statement creates a loop that consists of three optional expressions, enclosed in parentheses and separated by semicolons, followed by a statement (usually a block statement) to be executed in the loop.",
    tags: [
      "web-features:snapshot:ecmascript-1"
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
  walk_for_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ForStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "for",
    setup,
    should_ok_when_for_statement,
    r#"
        for (let i = 0; i < 9; i++) {
          console.log(i);
        }
      "#,
      1
  }
}
