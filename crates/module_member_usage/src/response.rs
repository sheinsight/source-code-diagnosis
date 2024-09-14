use beans::AstNode;
use napi_derive::napi;
use serde::Serialize;

#[napi(object)]
#[derive(Debug, Serialize, Clone)]
pub struct Response {
  pub lib_name: String,
  pub member_name: String,
  pub file_path: String,
  pub ast_node: AstNode,
}
