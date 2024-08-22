use crate::create_compat;

create_compat! {
  "./await.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_await_expression.push(walk_await_expression);
  },

  walk_await_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AwaitExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use crate::{assert_ok_count, syntax::operators::r#await::setup};

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
