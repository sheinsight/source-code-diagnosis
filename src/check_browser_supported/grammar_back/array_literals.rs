use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_array_expression.push(walk_array_expression);
  },
  compat {
    name: "array_literals",
    description: "数组字面量 ([1, 2, 3])",
    tags: [
      "web-features:array",
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_array_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ArrayExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "array_literals",
    setup,

    should_ok_when_use_array_literals,
    r#"
      const fruits = ["Apple", "Banana"];
    "#,
    1,

    should_ok_when_use_two_array_literals,
    r#"
      const fruits = ["Apple", "Banana"];
      const vegetables = ["Carrot", "Potato"];
    "#,
    2,
  }
}
