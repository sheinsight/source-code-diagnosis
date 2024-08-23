use crate::create_compat;

create_compat! {
  "./import_import_assertions.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_with_clause.push(walk_with_clause);
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
