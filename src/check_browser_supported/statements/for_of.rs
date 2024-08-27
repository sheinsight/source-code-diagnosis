use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_for_of_statement.push(walk_for_of_statement);
  },
  compat {
    name: "for_of",
    description: "for...of",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "38",
      chrome_android: "38",
      firefox: "13",
      firefox_android: "13",
      safari: "7",
      safari_ios: "7",
      edge: "12",
      node: "0.12.0",
      deno: "1.0",
    }
  },
  walk_for_of_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ForOfStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "for_of",
    setup,
    should_ok_when_for_of_statement,
    r#"
        const array1 = ['a', 'b', 'c'];
        for (const element of array1) {
          console.log(element);
        }
      "#,
      1
  }
}
