use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_continue_statement.push(walk_continue_statement);
  },
  compat {
    name: "continue",
    description: "The `continue` statement terminates execution of the statements in the current iteration of the current or labeled loop, and continues execution of the loop with the next iteration.",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
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
  walk_continue_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ContinueStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "continue",
    setup,
    should_ok_when_for_statement,
    r#"
    for (let i = 0; i < 10; i++) {
      if (i === 3) {
        continue;
      }
      text = text + i;
    }
    "#,
    1,
  }
}
