use crate::create_compat;

create_compat! {
  "./let.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_variable_declaration.push(walk_variable_declaration);
  },

  walk_variable_declaration,
  |ctx: &mut Context, it: &oxc_ast::ast::VariableDeclaration| {
    matches!(it.kind, oxc_ast::ast::VariableDeclarationKind::Let)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "let",
    setup,

    should_ok_when_let_declaration,
    r#"
      let x = 1;

      if (x === 1) {
        let x = 2;

        console.log(x);
        // Expected output: 2
      }

      console.log(x);
      // Expected output: 1
    "#,
    2,

    should_fail_when_var_declaration,
    r#"
      var x = 1;

      if (x === 1) {
        var x = 2;

        console.log(x);
      }

      console.log(x);
    "#,
    0,
  }
}
