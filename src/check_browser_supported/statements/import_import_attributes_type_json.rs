use oxc_ast::ast::ImportAttributeKey;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_with_clause.push(walk_with_clause);
  },
  compat {
    name: "import_import_attributes_type_json",
    description: "<code>with {type: 'json'}</code>",
    tags: ["web-features:js-modules"],
    support: {
      chrome: "123",
      chrome_android: "123",
      firefox: "-1",
      firefox_android: "-1",
      opera: "123",
      opera_android: "123",
      safari: "17.2",
      safari_ios: "17.2",
      edge: "123",
      oculus: "123",
      node: "20.10.0",
      deno: "1.37",
    }
  },
  walk_with_clause,
  |ctx: &mut Context, it: &oxc_ast::ast::WithClause| {
    let mut result = false;
    if it.attributes_keyword.name == "with" {
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
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "import_import_attributes_type_json",
    setup,

    should_ok_when_import_attributes_type_json,
    r#"
    import json from './data.json' with { type: 'json' };
    "#,
    1,

    should_ok_when_import_assertion_type_json,
    r#"
    import json from './data.json' assert { type: 'json' };
    "#,
    0,

    should_fail_when_import_attributes_type_css,
    r#"
    import json from './data.json' with { type: 'css' };
    "#,
    0,
  }
}
