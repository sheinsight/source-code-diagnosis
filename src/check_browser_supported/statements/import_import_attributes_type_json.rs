use oxc_ast::ast::ImportAttributeKey;

use crate::create_compat_2;

create_compat_2! {
    ImportImportAttributesTypeJson,
    compat {
        name: "statements.import.import_attributes.type_json",
        description: "<code>with {type: 'json'}</code>",
        mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#import_attributes",
        tags: ["web-features:js-modules"],
        support: {
            chrome: "123.0.0",
            chrome_android: "123.0.0",
            firefox: "-1",
            firefox_android: "-1",
            safari: "17.2.0",
            safari_ios: "17.2.0",
            edge: "123.0.0",
            node: "20.10.0",
            deno: "1.37.0",
        }
    },
    fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
        if let AstKind::ImportDeclaration(import_decl) = node.kind() {
            if let Some(with_clause) = &import_decl.with_clause {
                if with_clause.attributes_keyword.name == "with" {
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
  use super::ImportImportAttributesTypeJson;
  use crate::assert_source_seg;

  assert_source_seg! {
      should_ok_when_import_attributes_type_json: {
          setup: ImportImportAttributesTypeJson::default(),
          source_code: r#"
                import json from './data.json' with { type: 'json' };
            "#,
          eq: [
              r#"import json from './data.json' with { type: 'json' };"#,
          ],
          ne: []
      },

      should_fail_when_import_assertion_type_json: {
          setup: ImportImportAttributesTypeJson::default(),
          source_code: r#"
                import json from './data.json' assert { type: 'json' };
            "#,
          eq: [],
          ne: [
              r#"import json from './data.json' assert { type: 'json' };"#,
          ]
      },

      should_fail_when_import_attributes_type_css: {
          setup: ImportImportAttributesTypeJson::default(),
          source_code: r#"
                import json from './data.json' with { type: 'css' };
            "#,
          eq: [],
          ne: [
              r#"import json from './data.json' with { type: 'css' };"#,
          ]
      }
  }
}
