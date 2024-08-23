use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
  },
  compat {
    name: "operators_addition_assignment",
    description: "加法赋值运算符 (<code>x += y</code>)",
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
  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::Addition)
  }
}

#[cfg(test)]
mod tests {
  use crate::{assert_ok_count, syntax::operators::addition_assignment::setup};

  assert_ok_count! {
    "operators_addition_assignment",
    setup,

    should_ok_when_use_addition_assignment,
    r#"
      let a = 2;
      let b = 'hello';
      console.log((a += 3));
    "#,
    1,

    should_ok_when_use_addition_assignment_with_string,
    r#"
      let a = 'hello';
      let b = 'world';
      console.log((a += b));
    "#,
    1,
  }
}
