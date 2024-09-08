use oxc_semantic::{AstNode, AstNodes};

pub fn find_up_ast_node<'a>(
  nodes: &'a AstNodes,
  current_node: &'a AstNode,
  dep: u8,
) -> Option<&'a AstNode<'a>> {
  let mut dep = dep;

  if dep == 0 {
    return Some(current_node);
  }
  if let Some(node) = nodes.parent_node(current_node.id()) {
    dep -= 1;
    return find_up_ast_node(nodes, node, dep);
  } else {
    return None;
  }
}
