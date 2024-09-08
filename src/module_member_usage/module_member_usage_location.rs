use serde::Serialize;

#[napi(object)]
#[derive(Debug, Serialize, Clone)]
pub struct ModuleMemberUsageLocation {
  pub lib_name: String,
  pub member_name: String,
  pub file_path: String,
  pub ast_node: crate::utils::ast_node::AstNode,
}
