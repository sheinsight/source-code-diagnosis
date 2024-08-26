use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_numeric_literal.push(walk_numeric_literal);
  },
  compat {
    name: "numeric_separators",
    description: "数值分隔符 (1_000_000_000_000)",
    tags: [],
    support: {
      chrome: "75",
      chrome_android: "75",
      firefox: "70",
      firefox_android: "70",
      opera: "75",
      opera_android: "-1",
      safari: "13",
      safari_ios: "13",
      edge: "75",
      oculus: "75",
      node: "12.5.0",
      deno: "1.2",
    }
  },
  walk_numeric_literal,
  |ctx: &mut Context, it: &oxc_ast::ast::NumericLiteral| {
    it.raw.contains("_")
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "numeric_separators",
    setup,

    should_ok_when_use_numeric_separators,
    r#"
      1_000_000_000_000;
    "#,
    1,
  }
}
