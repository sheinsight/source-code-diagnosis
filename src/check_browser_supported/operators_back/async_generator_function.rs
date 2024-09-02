use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_function.push(walk_function);
  },
  compat {
    name: "operators_async_generator_function",
    description: "<code>async function*</code> 表达式",
    tags: [],
    support: {
      chrome: "63",
      chrome_android: "63",
      firefox: "55",
      firefox_android: "55",
      safari: "12",
      safari_ios: "12",
      edge: "63",
      node: "10.0.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function,flags: &oxc_semantic::ScopeFlags| {
    it.is_expression() && it.r#async && it.generator
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_async_generator_function",
    setup,

    should_ok_when_async_generator_function_declaration,
    r#"
      const asyncGenerator = async function* () {
        yield await Promise.resolve(1);
        yield await Promise.resolve(2);
        yield await Promise.resolve(3);
      };
    "#,
    1,

  }
}
