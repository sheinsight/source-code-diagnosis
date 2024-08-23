use crate::create_compat;

create_compat! {
  "./try_catch_optional_catch_binding.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_catch_clause.push(walk_catch_clause);
  },

  walk_catch_clause,
  |ctx: &mut Context, it: &oxc_ast::ast::CatchClause| {
    it.param.is_none()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "try_catch_optional_catch_binding",
    setup,
    should_ok_when_use_optional_catch_binding,
    r#"
    try {

    } catch {

    }
    "#,
    1,

    should_fail_when_no_use_optional_catch_binding,
    r#"
    try {

    } catch(a) {

    }
    "#,
    0,
  }
}
