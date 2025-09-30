use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  ClassesConstructor,
  compat {
    name: "classes.constructor",
    description: "constructor function",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/constructor",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "45",
      firefox_android: "45",
      safari: "9",
      safari_ios: "9",
      edge: "13",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, nodes: &AstNodes<'a>) -> bool {
    let node_kind = node.kind();

    if !matches!(node_kind, AstKind::MethodDefinition(_)) {
      return false;
    }

    let parent_node_id = nodes.parent_id(node.id());

    let parent_node = if let Some(parent_node_id) = parent_node_id {
      nodes.get_node(parent_node_id)
    } else {
      return false;
    };

    if !matches!(parent_node.kind(), AstKind::ClassBody(_)) {
      return false;
    }

    if let AstKind::MethodDefinition(method_definition) = node_kind {
      return method_definition
        .key
        .name()
        .map_or(false, |name| name == "constructor");
    }

    false
  }
}

#[cfg(test)]
mod tests {

  use super::ClassesConstructor;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_constructor:{
      setup: ClassesConstructor::default(),
      source_code: r#"
        class A { constructor() { } }
      "#,
      eq: [
        r#"constructor() { }"#
      ],
      ne: []
    }
  }
}
