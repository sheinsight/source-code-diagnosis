use oxc::ast::{ast::VariableDeclarationKind, AstKind};

use crate::create_compat;

create_compat! {
  Const,
  compat {
    name: "statements.const",
    description: "const 声明创建一个只读的常量。这不意味着常量指向的值是不可变的，只是变量标识符不能被重新赋值。",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/const",
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
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::VariableDeclaration(decl) if decl.kind == VariableDeclarationKind::Const)
  }
}

#[cfg(test)]
mod tests {
  use super::Const;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_const_declaration:{
      setup: Const::default(),
      source_code: r#"
        const number = 42;
        const obj = { key: 'value' };
        const arr = [1, 2, 3];
      "#,
      eq: [
        r#"const number = 42;"#,
        r#"const obj = { key: 'value' };"#,
        r#"const arr = [1, 2, 3];"#
      ],
      ne: [
        r#"let variable = 10;"#,
        r#"var oldVar = 'old';"#
      ]
    }
  }
}
