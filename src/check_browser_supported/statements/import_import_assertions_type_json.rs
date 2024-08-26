use oxc_ast::ast::ImportAttributeKey;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_with_clause.push(walk_with_clause);
  },
  compat {
    name: "import_import_assertions_type_json",
    description: "assert {type: 'json'} 语法",
    tags: ["web-features:js-modules"],
    support: {
      chrome: "91",
      chrome_android: "91",
      firefox: "91",
      firefox_android: "91",
      opera: "-1",
      opera_android: "-1",
      safari: "-1",
      safari_ios: "-1",
      edge: "91",
      oculus: "91",
      node: "-1",
      deno: "1.17",
    }
  },
  walk_with_clause,
  |ctx: &mut Context, it: &oxc_ast::ast::WithClause| {
    let mut result = false;
    if it.attributes_keyword.name == "assert" {
      for item in &it.with_entries {
        if let ImportAttributeKey::Identifier(key) = &item.key {
          if key.name == "type" && item.value.value == "json" {
            result = true;
          }
        }
      }
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count!(
    "import_import_assertions_type_json",
    setup,
    should_ok_when_import_assertions_type_json,
    r#"
    import styles from './styles.css' assert { type: 'json' };
    "#,
    1,
    should_fail_when_import_assertions_type_css,
    r#"
    import styles from './styles.css' assert { type: 'css' };
    "#,
    0,
  );
}
