use crate::create_compat;

create_compat! {
  "./import.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_import_expression.push(walk_import_expression);
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
