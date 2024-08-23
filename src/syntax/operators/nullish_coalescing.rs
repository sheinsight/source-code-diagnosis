use oxc_syntax::operator::LogicalOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_logical_expression.push(walk_logical_expression);
  },
  compat {
    name: "operators_nullish_coalescing",
    description: "空值合并运算符 (<code>??</code>)",
    tags: ["web-features:snapshot:ecmascript-2020"],
    support: {
      chrome: "80",
      chrome_android: "80",
      firefox: "72",
      firefox_android: "72",
      opera: "80",
      opera_android: "80",
      safari: "13.1",
      safari_ios: "13.1",
      edge: "80",
      oculus: "80",
      node: "14.0.0",
      deno: "1.0",
    }
  },
  walk_logical_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::LogicalExpression| {
    matches!(it.operator, LogicalOperator::Coalesce)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_nullish_coalescing",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    const foo = null ?? 'default string';
    console.log(foo);
    "#,
    1,
  }
}
