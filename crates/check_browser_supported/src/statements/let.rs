use oxc::ast::{ast::VariableDeclarationKind, AstKind};

use crate::create_compat;

create_compat! {
  Let,
  compat {
    name: "statements.let",
    description: "let 声明",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/let",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "44",
      firefox_android: "44",
      safari: "10",
      safari_ios: "10",
      edge: "14",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::VariableDeclaration(decl) if decl.kind == VariableDeclarationKind::Let)
  }
}

#[cfg(test)]
mod tests {
  use super::Let;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_let_declaration:{
      setup: Let::default(),
      source_code: r#"
        let x = 1;

        if (x === 1) {
          let x = 2;
          console.log(x);
        }

        console.log(x);
      "#,
      eq: [
        r#"let x = 1;"#,
        r#"let x = 2;"#,
      ],
      ne: []
    },

    should_fail_when_var_declaration:{
      setup: Let::default(),
      source_code: r#"
        var x = 1;

        if (x === 1) {
          var x = 2;
          console.log(x);
        }

        console.log(x);
      "#,
      eq: [],
      ne: [
        r#"var x = 1;"#,
        r#"var x = 2;"#,
      ]
    }
  }
}
