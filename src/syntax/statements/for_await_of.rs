use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_for_of_statement.push(walk_for_of_statement);
  },
  compat {
    name: "for_await_of",
    description: "for await...of",
    tags: ["web-features:snapshot:ecmascript-2018"],
    support: {
      chrome: "63",
      chrome_android: "63",
      firefox: "57",
      firefox_android: "57",
      opera: "63",
      opera_android: "63",
      safari: "12",
      safari_ios: "12",
      edge: "63",
      oculus: "63",
      node: "10.0.0",
      deno: "1.0",
    }
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
