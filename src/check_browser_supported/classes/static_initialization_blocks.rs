use crate::create_compat_2;

create_compat_2! {
  ClassesStaticInitializationBlocks,
  compat {
    name: "classes.static_initialization_blocks",
    description: "static initialization blocks",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Classes/Static_initialization_blocks",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "94.0.0",
      chrome_android: "94.0.0",
      firefox: "93.0.0",
      firefox_android: "93.0.0",
      safari: "16.4.0",
      safari_ios: "16.4.0",
      edge: "94.0.0",
      node: "16.11.0",
      deno: "1.14.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(
      node.kind(),
      AstKind::StaticBlock(_)
    )
  }
}

#[cfg(test)]
mod tests {

  use super::ClassesStaticInitializationBlocks;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_static_initialization_blocks:{
      setup: ClassesStaticInitializationBlocks::default(),
      source_code: r#"
        class A {
          static { }
        }
      "#,
      eq: [
        r#"static { }"#,
      ],
      ne: [

      ]
    },
  }
}
