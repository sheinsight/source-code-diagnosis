use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_parenthesized_expression.push(walk_parenthesized_expression);
  },
  compat {
    name: "operators_grouping",
    description: "分组运算符 <code>()</code>",
    tags: [
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
  walk_parenthesized_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ParenthesizedExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_grouping",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    console.log((2 + 3) * 4);
    "#,
    1,
  }
}
