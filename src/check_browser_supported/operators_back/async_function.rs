use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_function.push(walk_function);
      v.walk_arrow_function_expression.push(walk_arrow_function_expression);
  },
  compat {
    name: "operators_async_function",
    description: "<code>async function</code> 表达式",
    tags: ["web-features:async-await"],
    support: {
      chrome: "55",
      chrome_android: "55",
      firefox: "52",
      firefox_android: "52",
      safari: "10.1",
      safari_ios: "10.1",
      edge: "15",
      node: "7.6.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &oxc_semantic::ScopeFlags| {
    it.is_expression() && it.r#async && !it.generator
  },

  walk_arrow_function_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ArrowFunctionExpression| {
    it.r#async
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_async_function",
    setup,

    should_ok_when_async_function_declaration,
    r#"
      const asyncFunction = async function() {
      };
    "#,
    1,

    should_ok_when_async_function_declaration_with_arrow,
    r#"
      const asyncFunction = async () => {
      };
    "#,
    1,
  }
}
