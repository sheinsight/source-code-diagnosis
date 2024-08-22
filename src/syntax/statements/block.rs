use crate::create_compat;

create_compat! {
  "./block.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_block_statement.push(walk_block_statement);
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
