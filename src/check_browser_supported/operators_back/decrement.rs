use oxc_syntax::operator::UpdateOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_update_expression.push(walk_update_expression);
  },
  compat {
    name: "operators_decrement",
    description: "递减运算符 (<code>--</code>)",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "2",
      chrome_android: "2",
      firefox: "1",
      firefox_android: "1",
      safari: "4",
      safari_ios: "4",
      edge: "12",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_update_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::UpdateExpression| {
    matches!(it.operator, UpdateOperator::Decrement)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_decrement",
    setup,
    should_ok_when_use_decrement,
    r#"
      let x = 3;
      let y = x--;
      console.log(x);
      console.log(y);
    "#,
    1,
  }
}
