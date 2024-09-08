use serde::Serialize;

use crate::utils::ast_node::AstNode;

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct DangerStringLocation {
  pub raw_value: String,
  pub match_danger_string: String,
  pub file_path: String,
  pub ast_node: AstNode,
}
