use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
  "./if_else.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_if_statement.push(walk_if_statement);
  },

  walk_if_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::IfStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count!(
    "if_else",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    function testNum(a) {
      let result;
      if (a > 0) {
        result = 'positive';
      } else {
        result = 'NOT positive';
      }
      return result;
    }
    "#,
    1,
  );
}
