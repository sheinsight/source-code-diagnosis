use crate::create_compat_2;

create_compat_2! {
    ImportImportAssertions,
    compat {
        name: "statements.import.import_assertions",
        description: "使用 assert 语法的导入属性（前身为导入断言）",
        mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#import_assertions",
        tags: ["web-features:js-modules"],
        support: {
            chrome: "91.0.0",
            chrome_android: "91.0.0",
            firefox: "-1",
            firefox_android: "-1",
            safari: "-1",
            safari_ios: "-1",
            edge: "91.0.0",
            node: "16.14.0",
            deno: "1.17.0",
        }
    },
    fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
        if let AstKind::ImportDeclaration(import_decl) = node.kind() {
            if let Some(with_clause) = &import_decl.with_clause {
                return with_clause.attributes_keyword.name == "assert";
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
  use super::ImportImportAssertions;
  use crate::assert_source_seg;

  assert_source_seg! {
      should_ok_when_import_import_assertions: {
          setup: ImportImportAssertions::default(),
          source_code: r#"
                import json from './data.json' assert { type: 'json' };
            "#,
          eq: [
              r#"import json from './data.json' assert { type: 'json' };"#,
          ],
          ne: []
      },

      should_fail_when_import_import_attributes_type_css: {
          setup: ImportImportAssertions::default(),
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
