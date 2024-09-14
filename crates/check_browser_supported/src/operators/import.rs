use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  OperatorsImport,
  compat {
    name: "operators.import",
    description: "动态 import() 表达式",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/import",
    tags: ["web-features:js-modules"],
    support: {
      chrome: "63",
      chrome_android: "63",
      firefox: "67",
      firefox_android: "67",
      safari: "11",
      safari_ios: "11",
      edge: "63",
      node: "13.2.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::ImportExpression(_))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsImport;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_dynamic_import:{
      setup: OperatorsImport::default(),
      source_code: r#"
        import('./modules/myModule.js').then((module) => {
          // Do something with the module.
        });
      "#,
      eq: [
        r#"import('./modules/myModule.js')"#,
      ],
      ne: []
    }
  }
}
