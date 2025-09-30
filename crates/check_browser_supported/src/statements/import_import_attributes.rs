use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
    ImportImportAttributes,
    compat {
        name: "statements.import.import_attributes",
        description: "Import attributes (<code>with</code> syntax)",
        mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#import_attributes",
        tags: ["web-features:js-modules"],
        support: {
            chrome: "123",
            chrome_android: "123",
            firefox: "-1",
            firefox_android: "-1",
            safari: "17.2",
            safari_ios: "17.2",
            edge: "123",
            node: "20.10.0",
            deno: "1.37.0",
        }
    },
    fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
        if let AstKind::ImportDeclaration(import_decl) = node.kind() {
            if let Some(with_clause) = &import_decl.with_clause {
                return with_clause.attributes_keyword.name == "with";
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
  use super::ImportImportAttributes;
  use crate::assert_source_seg;

  assert_source_seg! {
      should_ok_when_import_attributes: {
          setup: ImportImportAttributes::default(),
          source_code: r#"
                import json from './data.json' with { type: 'json' };
            "#,
          eq: [
              r#"import json from './data.json' with { type: 'json' };"#,
          ],
          ne: []
      },

      should_fail_when_import_assertions: {
          setup: ImportImportAttributes::default(),
          source_code: r#"
                import json from './data.json' assert { type: 'json' };
            "#,
          eq: [],
          ne: [
              r#"import json from './data.json' assert { type: 'json' };"#,
          ]
      },

      should_fail_when_normal_import: {
          setup: ImportImportAttributes::default(),
          source_code: r#"
                import json from './data.json';
            "#,
          eq: [],
          ne: [
              r#"import json from './data.json';"#,
          ]
      }
  }
}
