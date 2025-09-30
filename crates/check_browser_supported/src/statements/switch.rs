use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  Switch,
  compat {
    name: "statements.switch",
    description: "switch 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/switch",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "18",
      firefox: "1",
      firefox_android: "4",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::SwitchStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::Switch;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_switch_statement:{
      setup: Switch::default(),
      source_code: r#"
        const expr = 'Papayas';
        switch (expr) {
          case 'Oranges':
            console.log('Oranges are $0.59 a pound.');
            break;
          case 'Mangoes':
          case 'Papayas':
            console.log('Mangoes and papayas are $2.79 a pound.');
            break;
          default:
            console.log(`Sorry, we are out of ${expr}.`);
        }
      "#,
      eq: [
        r#"switch (expr) {
          case 'Oranges':
            console.log('Oranges are $0.59 a pound.');
            break;
          case 'Mangoes':
          case 'Papayas':
            console.log('Mangoes and papayas are $2.79 a pound.');
            break;
          default:
            console.log(`Sorry, we are out of ${expr}.`);
        }"#
      ],
      ne: []
    }
  }
}
