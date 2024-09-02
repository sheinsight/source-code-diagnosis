use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
  },
  compat {
    name: "operators_nullish_coalescing_assignment",
    description: "空值合并赋值运算符 (<code>x ??= y</code>)",
    tags: [
      "web-features:snapshot:ecmascript-2020"
    ],
    support: {
      chrome: "85",
      chrome_android: "85",
      firefox: "79",
      firefox_android: "79",
      safari: "14",
      safari_ios: "14",
      edge: "85",
      node: "15.0.0",
      deno: "1.2",
    }
  },
  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::LogicalNullish)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_nullish_coalescing_assignment",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    let x = null;
    let y = 5;
    x ??= y;
    console.log(x);
    "#,
    1,
  }
}
