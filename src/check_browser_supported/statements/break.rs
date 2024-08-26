use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_break_statement.push(walk_break_statement);
  },
  compat {
    name: "break",
    description: "The `break` statement terminates the current loop, switch, or label statement and transfers program control to the statement following the terminated statement.",
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
  walk_break_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::BreakStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "break",
    setup,

    should_ok_when_while,
    r#"
  let i = 0;

  while (i < 6) {
    if (i === 3) {
      break;
    }
    i = i + 1;
  }

  console.log(i);
    "#,
    1,
  }
}
