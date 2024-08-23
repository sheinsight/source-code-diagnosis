use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_import_expression.push(walk_import_expression);
  },
  compat {
    name: "operators_import_options_parameter",
    description: "<code>options</code> 参数",
    tags: [],
    support: {
      chrome: "91",
      chrome_android: "91",
      firefox: "-1",
      firefox_android: "-1",
      opera: "-1",
      opera_android: "-1",
      safari: "15",
      safari_ios: "15",
      edge: "91",
      oculus: "91",
      node: "17.5.0",
      deno: "1.17",
    }
  },
  walk_import_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ImportExpression| {
    !it.arguments.is_empty()
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_import_options_parameter",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    import('./module.js', { assert: { type: 'json' } })
    "#,
    1,
  }
}
