use crate::create_compat_2;

create_compat_2! {
  DebuggerStatement,
  compat {
    name: "statements.debugger",
    description: "debugger 语句调用任何可用的调试功能，例如设置断点。如果没有可用的调试功能，该语句不会产生任何效果。",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/debugger",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "5",
      chrome_android: "18",
      firefox: "1",
      firefox_android: "4",
      safari: "5",
      safari_ios: "5",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::DebuggerStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::DebuggerStatement;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_debugger_statement:{
      setup: DebuggerStatement::default(),
      source_code: r#"
        function test() {
          debugger;
          console.log("After debugger");
        }
      "#,
      eq: [
        r#"debugger;"#,
      ],
      ne: []
    }
  }
}
