use oxc_ast::ast::ImportAttributeKey;

use crate::create_compat;

create_compat! {
  "./import_import_attributes_type_css.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_with_clause.push(walk_with_clause);
  },

  walk_with_clause,
  |ctx: &mut Context, it: &oxc_ast::ast::WithClause| {
    let mut result = false;
    if it.attributes_keyword.name == "with" {
      for item in &it.with_entries {
        if let ImportAttributeKey::Identifier(key) = &item.key {
          if key.name == "type" && item.value.value == "css" {
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

  assert_ok_count! {
    "import_import_attributes_type_css",
    setup,

    should_ok_when_import_import_attributes_type_css,
    r#"
    import json from './data.json' with { type: 'css' };
    "#,
    1,

    should_ok_when_import_attributes_type_json,
    r#"
    import json from './data.json' with { type: 'json' };
    "#,
    0,

    should_ok_when_import_assertion_type_css,
    r#"
    import json from './data.json' assert { type: 'css' };
    "#,
    0,
  }
}
