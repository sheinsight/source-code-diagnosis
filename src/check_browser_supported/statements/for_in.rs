use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_for_in_statement.push(walk_for_in_statement);
  },
  compat {
    name: "for_in",
    description: "for...in",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "2",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_for_in_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ForInStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
      "for_in",
      setup,
      should_ok_when_for_in_statement,
      r#"
const object = { a: 1, b: 2, c: 3 };
for (const property in object) {
  console.log(`${property}: ${object[property]}`);
}
"#,
  1
    }
}
