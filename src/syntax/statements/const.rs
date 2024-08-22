use oxc_ast::ast::VariableDeclarationKind;

use crate::create_compat;

create_compat! {
  "./const.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_variable_declaration.push(walk_variable_declaration);
  },
  walk_variable_declaration,
  |ctx: &mut Context, it: &oxc_ast::ast::VariableDeclaration| {
    matches!(it.kind, VariableDeclarationKind::Const)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "const",
    setup,
    should_ok_when_const_declaration,
    r#"
    const number = 42;
    "#,
    1,
  }
}
