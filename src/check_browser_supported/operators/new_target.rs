use crate::create_compat_2;

create_compat_2! {
  OperatorsNewTarget,
  compat {
    name: "operators.new_target",
    description: "<code>new.target</code>",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/new.target",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "46.0.0",
      chrome_android: "46.0.0",
      firefox: "41.0.0",
      firefox_android: "41.0.0",
      safari: "11.0.0",
      safari_ios: "11.0.0",
      edge: "13.0.0",
      node: "5.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::MetaProperty(meta) if meta.meta.name == "new" && meta.property.name == "target")
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsNewTarget;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_new_target:{
      setup: OperatorsNewTarget::default(),
      source_code: r#"
        function Foo() {
          if (!new.target) {
            throw new Error("Foo() must be called with new");
          }
          console.log("Foo instantiated with new");
        }
      "#,
      eq: [
        r#"new.target"#,
      ],
      ne: []
    }
  }
}
