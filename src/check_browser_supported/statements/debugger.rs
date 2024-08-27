use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_debugger_statement.push(walk_debugger_statement);
  },
  compat {
    name: "debugger",
    description: "The `debugger` statement invokes any available debugging functionality, such as setting a breakpoint. If no debugging functionality is available, this statement has no effect.",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "5",
      chrome_android: "5",
      firefox: "1",
      firefox_android: "1",
      safari: "5",
      safari_ios: "5",
      edge: "12",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_debugger_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::DebuggerStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "debugger",
    setup,
    should_ok_when_debugger_statement,
    r#"
    debugger;
    "#,
    1,
  }
}
