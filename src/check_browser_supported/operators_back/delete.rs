use oxc_syntax::operator::UnaryOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_unary_expression.push(walk_unary_expression);
  },
  compat {
    name: "delete",
    description: "delete 运算符",
    tags: ["web-features:snapshot:ecmascript-1"],
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
  walk_unary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::UnaryExpression| {
    matches!(it.operator, UnaryOperator::Delete)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "delete",
    setup,


    should_ok_when_delete_object_key,
    r#"
      const myObject = {x: 1, y: 2};
      delete myObject.x;
    "#,
    1,

  }
}
