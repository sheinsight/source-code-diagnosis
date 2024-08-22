use crate::create_compat;

create_compat! {
  "./debugger.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_debugger_statement.push(walk_debugger_statement);
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
