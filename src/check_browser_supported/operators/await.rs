use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_await_expression.push(walk_await_expression);
  },
  compat {
    name: "operators_await",
    description: "await 运算符",
    tags: ["web-features:async-await"],
    support: {
      chrome: "55",
      chrome_android: "55",
      firefox: "52",
      firefox_android: "52",
      opera: "55",
      opera_android: "55",
      safari: "10.1",
      safari_ios: "10.1",
      edge: "14",
      oculus: "55",
      node: "7.6.0",
      deno: "1.0",
    }
  },
  walk_await_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AwaitExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count, check_browser_supported::operators::r#await::setup,
  };

  assert_ok_count! {
    "operators_await",
    setup,

    should_ok_when_use_await,
    r#"
      const response = await fetch('https://api.example.com/data');
      const data = await response.json();
    "#,
    2,

    should_ok_when_use_await_in_if_statement,
    r#"
      if (true) {
        const response = await fetch('https://api.example.com/data');
        const data = await response.json();
      }
    "#,
    2,

    should_ok_when_use_await_in_try_catch_statement,
    r#"
      try {
        const response = await fetch('https://api.example.com/data');
        const data = await response.json();
      } catch (error) {
        console.error('Error:', error);
      }
    "#,
    2,
  }
}
