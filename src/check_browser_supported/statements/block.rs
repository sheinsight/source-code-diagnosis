use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_block_statement.push(walk_block_statement);
  },
  compat {
    name: "block",
    description: "A block statement (or compound statement in other languages) is used to group zero or more statements. The block is delimited by a pair of curly brackets.",
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
  walk_block_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::BlockStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "block",
    setup,

    should_ok_when_has_one_block,
    r#"
    var x = 1;
    let y = 1;
    if (true) {
      var x = 2;
      let y = 2;
    }
    console.log(x);
    console.log(y);
    "#,
    1,

    should_ok_when_nested_block,
    r#"
    var x = 1;
    let y = 1;
    if (true) {
      var x = 2;
      let y = 2;
      if (true) {
        console.log('two')
      }
    }
    console.log(x);
    console.log(y);
    "#,
    2,
  }
}
