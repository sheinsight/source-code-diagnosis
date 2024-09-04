use crate::create_compat_2;

create_compat_2! {
  BreakStatement,
  compat {
    name: "statements.break",
    description: "break 语句用于终止当前循环、switch 或标签语句,并将程序控制权转移到被终止语句后面的语句。",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/break",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1.0.0",
      chrome_android: "1.0.0",
      firefox: "1.0.0",
      firefox_android: "1.0.0",
      safari: "1.0.0",
      safari_ios: "1.0.0",
      edge: "12.0.0",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::BreakStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::BreakStatement;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_break_in_while:{
      setup: BreakStatement::default(),
      source_code: r#"
        let i = 0;
        while (i < 6) {
          if (i === 3) {
            break;
          }
          i = i + 1;
        }
        console.log(i);
      "#,
      eq: [
        r#"break;"#,
      ],
      ne: []
    },

    should_ok_when_use_break_in_switch:{
      setup: BreakStatement::default(),
      source_code: r#"
        switch (expr) {
          case 'Oranges':
            console.log('Oranges are $0.59 a pound.');
            break;
          case 'Apples':
            console.log('Apples are $0.32 a pound.');
            break;
          default:
            console.log('Sorry, we are out of ' + expr + '.');
        }
      "#,
      eq: [
        r#"break;"#,
        r#"break;"#,
      ],
      ne: []
    }
  }
}
