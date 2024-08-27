use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_catch_clause.push(walk_catch_clause);
  },
  compat {
    name: "try_catch_optional_catch_binding",
    description: "Optional catch binding",
    tags: [],
    support: {
      chrome: "66",
      chrome_android: "66",
      firefox: "58",
      firefox_android: "58",
      safari: "11.1",
      safari_ios: "11.1",
      edge: "66",
      node: "10.0.0",
      deno: "1.0",
    }
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
