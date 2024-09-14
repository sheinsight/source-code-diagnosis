use oxc_ast::{ast::ImportAttributeKey, AstKind};

use crate::create_compat;

create_compat! {
    ImportImportAssertionsTypeCSS,
    compat {
        name: "statements.import.import_assertions.type_css",
        description: "assert {type: 'css'} 语法",
        mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#import_assertions",
        tags: ["web-features:js-modules"],
        support: {
            chrome: "93",
            chrome_android: "93",
            firefox: "-1",
            firefox_android: "-1",
            safari: "-1",
            safari_ios: "-1",
            edge: "93",
            node: "-1",
            deno: "-1",
        }
    },
    fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
        if let AstKind::ImportDeclaration(import_decl) = node.kind() {
            if let Some(with_clause) = &import_decl.with_clause {
                if with_clause.attributes_keyword.name == "assert" {
                    for item in &with_clause.with_entries {
                        if let ImportAttributeKey::Identifier(key) = &item.key {
                            if key.name == "type" && item.value.value == "css" {
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
  use super::ImportImportAssertionsTypeCSS;
  use crate::assert_source_seg;

  assert_source_seg! {
      should_ok_when_import_assertions_type_css: {
          setup: ImportImportAssertionsTypeCSS::default(),
          source_code: r#"
                import styles from './styles.css' assert { type: 'css' };
            "#,
          eq: [
              r#"import styles from './styles.css' assert { type: 'css' };"#,
          ],
          ne: []
      },

      should_fail_when_import_assertions_type_json: {
          setup: ImportImportAssertionsTypeCSS::default(),
          source_code: r#"
                import styles from './styles.css' assert { type: 'json' };
            "#,
          eq: [],
          ne: [
              r#"import styles from './styles.css' assert { type: 'json' };"#,
          ]
      },

      should_fail_when_import_attributes_type_css: {
          setup: ImportImportAssertionsTypeCSS::default(),
          source_code: r#"
                import styles from './styles.css' with { type: 'css' };
            "#,
          eq: [],
          ne: [
              r#"import styles from './styles.css' with { type: 'css' };"#,
          ]
      }
  }
}
