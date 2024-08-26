use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_binary_expression.push(walk_binary_expression);
  },
  compat {
    name: "operators_in",
    description: "in 运算符",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "4",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::In)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_in",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    const car = { make: 'Honda', model: 'Accord', year: 1998 };
    console.log('make' in car);
    console.log('color' in car);
    "#,
    2,
  }
}
