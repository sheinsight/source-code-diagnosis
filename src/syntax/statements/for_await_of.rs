use crate::create_compat;

create_compat! {
  "./for_await_of.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_for_of_statement.push(walk_for_of_statement);
  },
  walk_for_of_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ForOfStatement| {
    it.r#await
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "for_await_of",
    setup,
    should_ok_when_for_await_of_statement,
    r#"
async function* foo() {
  yield 1;
  yield 2;
}
(async function () {
  for await (const num of foo()) {
    console.log(num);
    // Expected output: 1

    break; // Closes iterator, triggers return
  }
})();
    "#,
    1,
  }
}
