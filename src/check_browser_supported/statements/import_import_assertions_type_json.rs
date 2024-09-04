use oxc_ast::ast::ImportAttributeKey;

use crate::create_compat_2;

create_compat_2! {
    ImportImportAssertionsTypeJson,
    compat {
        name: "import_import_assertions_type_json",
        description: "assert {type: 'json'} 语法",
        mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#import_assertions",
        tags: ["web-features:js-modules"],
        support: {
            chrome: "91.0.0",
            chrome_android: "91.0.0",
            firefox: "91.0.0",
            firefox_android: "91.0.0",
            safari: "-1",
            safari_ios: "-1",
            edge: "91.0.0",
            node: "-1",
            deno: "1.17.0",
        }
    },
    fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
        if let AstKind::ImportDeclaration(import_decl) = node.kind() {
            if let Some(with_clause) = &import_decl.with_clause {
                if with_clause.attributes_keyword.name == "assert" {
                    for item in &with_clause.with_entries {
                        if let ImportAttributeKey::Identifier(key) = &item.key {
                            if key.name == "type" && item.value.value == "json" {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
  use super::ImportImportAssertionsTypeJson;
  use crate::assert_source_seg;

  assert_source_seg! {
      should_ok_when_import_assertions_type_json: {
          setup: ImportImportAssertionsTypeJson::default(),
          source_code: r#"
                import styles from './styles.css' assert { type: 'json' };
            "#,
          eq: [
              r#"import styles from './styles.css' assert { type: 'json' };"#,
          ],
          ne: []
      },

      should_fail_when_import_assertions_type_css: {
          setup: ImportImportAssertionsTypeJson::default(),
          source_code: r#"
                import styles from './styles.css' assert { type: 'css' };
            "#,
          eq: [],
          ne: [
              r#"import styles from './styles.css' assert { type: 'css' };"#,
          ]
      },

      should_fail_when_no_assert_clause: {
          setup: ImportImportAssertionsTypeJson::default(),
          source_code: r#"
                import styles from './styles.css';
            "#,
          eq: [],
          ne: []
      }
  }
}
