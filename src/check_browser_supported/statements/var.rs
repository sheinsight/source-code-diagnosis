use oxc_ast::ast::VariableDeclarationKind;

use crate::create_compat_2;

create_compat_2! {
  VarDeclaration,
  compat {
    name: "statements.var",
    description: "var 声明",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/var",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::VariableDeclaration(decl) if decl.kind == VariableDeclarationKind::Var)
  }
}

#[cfg(test)]
mod tests {
  use super::VarDeclaration;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_var_declaration:{
      setup: VarDeclaration::default(),
      source_code: r#"
        var x = 1;
        if (x === 1) {
          var x = 2;
          console.log(x);
        }
        console.log(x);
      "#,
      eq: [
        r#"var x = 1;"#,
        r#"var x = 2;"#,
      ],
      ne: []
    },

    should_fail_when_use_let_declaration:{
      setup: VarDeclaration::default(),
      source_code: r#"
        let x = 1;
        if (x === 1) {
          let x = 2;
          console.log(x);
        }
        console.log(x);
      "#,
      eq: [],
      ne: [
        r#"let x = 1;"#,
        r#"let x = 2;"#,
      ]
    }
  }
}
