use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  OperatorsImportMeta,
  compat {
    name: "operators.import_meta",
    description: "<code>import.meta</code>",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/import.meta",
    tags: ["web-features:js-modules"],
    support: {
      chrome: "64",
      chrome_android: "64",
      firefox: "62",
      firefox_android: "62",
      safari: "11.1",
      safari_ios: "12",
      edge: "64",
      node: "10.4.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::MetaProperty(meta) if meta.meta.name == "import" && meta.property.name == "meta")
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsImportMeta;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_import_meta:{
      setup: OperatorsImportMeta::default(),
      source_code: r#"
        const relativeURL = import.meta.url;
      "#,
      eq: [
        r#"import.meta"#
      ],
      ne: []
    }
  }
}
