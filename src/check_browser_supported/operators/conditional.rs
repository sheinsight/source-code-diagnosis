use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_conditional_expression.push(walk_conditional_expression);
  },
  compat {
    name: "operators_conditional",
    description: "条件（三元）运算符 (<code>c ? t : f</code>)",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "3",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_conditional_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ConditionalExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_conditional",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
      const age = 26;
      const beverage = age >= 21 ? "Beer" : "Juice";
      console.log(beverage);
    "#,
    1
  }
}
