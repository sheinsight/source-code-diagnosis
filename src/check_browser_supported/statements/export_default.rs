use crate::create_compat_2;

create_compat_2! {
  ExportDefault,
  compat {
    name: "statements.export_default",
    description: "export 语句中的 default 关键字",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/export",
    tags: ["web-features:js-modules"],
    support: {
      chrome: "61.0.0",
      chrome_android: "61.0.0",
      firefox: "60.0.0",
      firefox_android: "60.0.0",
      safari: "10.1.0",
      safari_ios: "10.1.0",
      edge: "16.0.0",
      node: "13.2.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::ExportDefaultDeclaration(_))
  }
}

#[cfg(test)]
mod tests {
  use super::ExportDefault;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_export_default_declaration:{
      setup: ExportDefault::default(),
      source_code: r#"
        export default function() {}
        export default class {}
        export default 1;
      "#,
      eq: [
        r#"export default function() {}"#,
        r#"export default class {}"#,
        r#"export default 1;"#,
      ],
      ne: [
        r#"export { default } from 'module';"#,
      ]
    }
  }
}
