use oxc_ast::AstKind;
use oxc_semantic::{AstNode, AstNodes};

use crate::check_browser_supported::compat::{Compat, CompatHandler, Support};

pub struct ClassesConstructor {
  pub compat: Compat,
}

impl Default for ClassesConstructor {
  fn default() -> Self {
    Self {
          compat: Compat {
              name: "default_parameters_destructured_parameter_with_default_value_assignment".to_string(),
              description: "destructured parameter with default value assignment".to_string(),
          tags: vec![
              "web-features:default-parameters-destructured-parameter-with-default-value-assignment".to_string(),
              "web-features:snapshot:ecmascript-2015".to_string()
          ],
          support: Support {
              chrome: "49.0.0".to_string(),
              chrome_android: "49".to_string(),
                  firefox: "41".to_string(),
                  firefox_android: "41".to_string(),
                  safari: "10".to_string(),
                  safari_ios: "10".to_string(),
              edge: "14".to_string(),
              node: "6.0.0".to_string(),
              deno: "1.0".to_string(),
          }
          }
      }
  }
}

impl CompatHandler for ClassesConstructor {
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

  fn get_compat(&self) -> &Compat {
    &self.compat
  }
}
