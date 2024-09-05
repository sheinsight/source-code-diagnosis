use crate::create_compat_2;

create_compat_2! {
  BlockStatement,
  compat {
    name: "statements.block",
    description: "块语句（或在其他语言中称为复合语句）用于将零个或多个语句组合在一起。块由一对花括号界定。",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/block",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::BlockStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::BlockStatement;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_has_one_block:{
      setup: BlockStatement::default(),
      source_code: r#"
        var x = 1;
        let y = 1;
        if (true) {
          var x = 2;
          let y = 2;
        }
        console.log(x);
        console.log(y);
      "#,
      eq: [
        r#"{
          var x = 2;
          let y = 2;
        }"#
      ],
      ne: []
    },

    should_ok_when_nested_block:{
      setup: BlockStatement::default(),
      source_code: r#"
        var x = 1;
        let y = 1;
        if (true) {
          var x = 2;
          let y = 2;
          if (true) {
            console.log('two')
          }
        }
        console.log(x);
        console.log(y);
      "#,
      eq: [
        r#"{
          var x = 2;
          let y = 2;
          if (true) {
            console.log('two')
          }
        }"#,
        r#"{
            console.log('two')
          }"#
      ],
      ne: []
    }
  }
}
