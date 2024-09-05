use crate::create_compat_2;

create_compat_2! {
    ImportOptionsParameter,
    compat {
        name: "operators.import.options_parameter",
        description: "<code>options</code> 参数",
        mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#import_with_options",
        tags: ["web-features:js-modules"],
        support: {
            chrome: "91",
            chrome_android: "91",
            firefox: "-1",
            firefox_android: "-1",
            safari: "15",
            safari_ios: "15",
            edge: "91",
            node: "17.5.0",
            deno: "1.17.0",
        }
    },
    fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
        if let AstKind::ImportExpression(import_expr) = node.kind() {
            !import_expr.arguments.is_empty()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
  use super::ImportOptionsParameter;
  use crate::assert_source_seg;

  assert_source_seg! {
      should_ok_when_use_import_with_options: {
          setup: ImportOptionsParameter::default(),
          source_code: r#"
                import('./module.js', { assert: { type: 'json' } });
            "#,
          eq: [
              r#"import('./module.js', { assert: { type: 'json' } })"#,
          ],
          ne: []
      },

      should_ng_when_use_import_without_options: {
          setup: ImportOptionsParameter::default(),
          source_code: r#"
                import('./module.js');
            "#,
          eq: [],
          ne: [
              r#"('./module.js')"#,
          ]
      }
  }
}
