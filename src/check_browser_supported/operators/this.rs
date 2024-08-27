use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_this_expression.push(walk_this_expression);
  },
  compat {
    name: "operators_this",
    description: "The `this` keyword refers to the object it belongs to. It has different values depending on where it is used.",
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
  walk_this_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ThisExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_this",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    console.log(this === window);
    "#,
    1
  }
}
