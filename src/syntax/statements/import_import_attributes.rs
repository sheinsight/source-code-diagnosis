use crate::create_compat;

create_compat! {
  "./import_import_attributes.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_with_clause.push(walk_with_clause);
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
