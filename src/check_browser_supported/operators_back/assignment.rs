use oxc_ast::ast::{AssignmentExpression, VariableDeclarator};
use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_variable_declarator.push(walk_variable_declarator);
      v.walk_assignment_expression.push(walk_assignment_expression);
  },
  compat {
    name: "operators_assignment",
    description: "赋值运算符 (<code>x = y</code>)",
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
  walk_variable_declarator,
  |ctx: &mut Context, it: &VariableDeclarator| {
    it.init.is_some()
  },

  walk_assignment_expression,
  |ctx: &mut Context, it: &AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::Assign)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count, check_browser_supported::operators::assignment::setup,
  };

  assert_ok_count! {
    "operators_assignment",
    setup,

    should_ok_when_basic_assignment,
    r#"
      let x = 5;
    "#,
    1,

    should_ok_when_continuous_assignment,
    r#"
      let a, b, c;
      a = b = c = 2;
    "#,
    3,

    should_ok_when_deconstruct_assignment,
    r#"
      let [d, e] = [1, 2];
      console.log(d, e);
    "#,
    1,

    should_ok_when_object_deconstruct_assignment,
    r#"
      let {f, g} = {f: 3, g: 4};
      console.log(f, g);
    "#,
    1,

    should_ok_when_assignment_with_expression,
    r#"
      let h = 5;
      let i = 6;
      console.log((h = i));
    "#,
    3,
  }
}
