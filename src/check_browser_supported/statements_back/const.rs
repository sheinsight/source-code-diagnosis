use oxc_ast::ast::VariableDeclarationKind;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_variable_declaration.push(walk_variable_declaration);
  },
  compat {
    name: "const",
    description: "The `const` declaration creates a read-only reference to a value. It does not mean the value it holds is immutable, just that the variable identifier cannot be reassigned.",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "21",
      chrome_android: "21",
      firefox: "36",
      firefox_android: "36",
      safari: "5.1",
      safari_ios: "5.1",
      edge: "12",
      node: "6.0.0",
      deno: "1.0",
    }
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
