use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_with_clause.push(walk_with_clause);
  },
  compat {
    name: "import_import_attributes",
    description: "Import attributes (<code>with</code> syntax)",
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
      deno: "1.37",
    }
  },
  walk_with_clause,
  |ctx: &mut Context, it: &oxc_ast::ast::WithClause| {
    it.attributes_keyword.name == "with"
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "import_import_attributes",
    setup,

    should_ok_when_import_attributes,
    r#"
    import json from './data.json' with { type: 'json' };
    "#,
    1,

    should_fail_when_import_attributes_type_css,
    r#"
    import json from './data.json' assert { type: 'css' };
    "#,
    0,
  }
}
