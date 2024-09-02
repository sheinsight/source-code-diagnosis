use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_with_clause.push(walk_with_clause);
  },
  compat {
    name: "import_import_assertions",
    description: "使用 assert 语法的导入属性（前身为导入断言）",
    tags: ["web-features:js-modules"],
    support: {
      chrome: "91",
      chrome_android: "91",
      firefox: "-1",
      firefox_android: "-1",
      safari: "-1",
      safari_ios: "-1",
      edge: "91",
      node: "16.14.0",
      deno: "1.17",
    }
  },
  walk_with_clause,
  |ctx: &mut Context, it: &oxc_ast::ast::WithClause| {
    it.attributes_keyword.name == "assert"
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "import_import_assertions",
    setup,

    should_ok_when_import_import_assertions,
    r#"
    import json from './data.json' assert { type: 'json' };
    "#,
    1,

    should_fail_when_import_import_attributes_type_css,
    r#"
    import json from './data.json' with { type: 'css' };
    "#,
    0,
  }
}
