use oxc_ast::ast::{Expression, MemberExpression};

use crate::create_compat_2;

create_compat_2! {
  OperatorsImportMetaResolve,
  compat {
    name: "operators_import_meta_resolve",
    description: "<code>import.meta.resolve</code>",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/import.meta/resolve",
    tags: ["web-features:js-modules"],
    support: {
      chrome: "105.0.0",
      chrome_android: "105.0.0",
      firefox: "106.0.0",
      firefox_android: "106.0.0",
      safari: "16.4.0",
      safari_ios: "16.4.0",
      edge: "105.0.0",
      node: "20.6.0",
      deno: "1.24.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::MemberExpression(expr) = node.kind() {
      if let MemberExpression::StaticMemberExpression(expr) = expr {
        return matches!(
          &expr.object, Expression::MetaProperty(meta)
          if meta.meta.name == "import" && meta.property.name == "meta" && expr.property.name == "resolve"
        )
      }
    }
false

  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsImportMetaResolve;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_import_meta_resolve: {
      setup: OperatorsImportMetaResolve::default(),
      source_code: r#"
        const relativeURL = import.meta.resolve('./module.js');
      "#,
      eq: [
        r#"import.meta.resolve('./module.js')"#,
      ],
      ne: []
    }
  }
}
