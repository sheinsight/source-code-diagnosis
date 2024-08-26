use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_import_expression.push(walk_import_expression);
  },
  compat {
    name: "operators_import",
    description: "",
    tags: [],
    support: {
      chrome: "63",
      chrome_android: "63",
      firefox: "67",
      firefox_android: "67",
      opera: "63",
      opera_android: "63",
      safari: "11.1",
      safari_ios: "11.1",
      edge: "63",
      oculus: "63",
      node: "13.2.0",
      deno: "1.0",
    }
  },
  walk_import_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ImportExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_import",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    import('./modules/myModule.js')
    "#,
    1,
  }
}
