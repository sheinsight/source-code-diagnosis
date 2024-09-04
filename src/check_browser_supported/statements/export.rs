use crate::create_compat_2;

create_compat_2! {
    Export,
    compat {
        name: "export",
        description: "The `export` statement is used when creating JavaScript modules to export functions, objects, or primitive values from the module so they can be used by other programs with the `import` statement.",
        mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/export",
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
        matches!(
            node.kind(),
            AstKind::ExportNamedDeclaration(_) | AstKind::ExportDefaultDeclaration(_) | AstKind::ExportAllDeclaration(_)
        )
    }
}

#[cfg(test)]
mod tests {
  use super::Export;
  use crate::assert_source_seg;

  assert_source_seg! {
      should_ok_when_export_named_declaration: {
          setup: Export::default(),
          source_code: r#"
                export const a = 1;
                export function foo() {}
                export { bar };
            "#,
          eq: [
              r#"export const a = 1;"#,
              r#"export function foo() {}"#,
              r#"export { bar };"#,
          ],
          ne: []
      },

      should_ok_when_export_default_declaration: {
          setup: Export::default(),
          source_code: r#"
                export default 1;
                export default function foo() {}
                export default class Bar {}
            "#,
          eq: [
              r#"export default 1;"#,
              r#"export default function foo() {}"#,
              r#"export default class Bar {}"#,
          ],
          ne: []
      },

      should_ok_when_export_all_declaration: {
          setup: Export::default(),
          source_code: r#"
                export * from 'module';
                export * as name from 'module';
            "#,
          eq: [
              r#"export * from 'module';"#,
              r#"export * as name from 'module';"#,
          ],
          ne: []
      }
  }
}
