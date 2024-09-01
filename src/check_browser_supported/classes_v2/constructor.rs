use crate::create_compat_2;

create_compat_2! {
  ClassesConstructor,
  compat {
    name: "classes_constructor",
    description: "constructor function",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49.0.0",
      chrome_android: "49.0.0",
      firefox: "45.0.0",
      firefox_android: "45.0.0",
      safari: "9.0.0",
      safari_ios: "9.0.0",
      edge: "13.0.0",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, node: &AstNode<'a>, nodes: &AstNodes<'a>) -> bool {
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
