use oxc_ast::ast::VariableDeclarationKind;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_variable_declaration.push(walk_variable_declaration);
  },
  compat {
    name: "var",
    description: "var 声明",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "3",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_variable_declaration,
  |ctx: &mut Context, it: &oxc_ast::ast::VariableDeclaration| {
    matches!(it.kind, VariableDeclarationKind::Var)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "var",
    setup,
    should_ok_when_use_var_declaration,
    r#"
    var x = 1;

    if (x === 1) {
      var x = 2;

      console.log(x);
    }

    console.log(x);
    "#,
    2,

    should_fail_when_use_let_declaration,
    r#"
    let x = 1;

    if (x === 1) {
      let x = 2;

      console.log(x);
    }

    console.log(x);
    "#,
    0
  }
}
