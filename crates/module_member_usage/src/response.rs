use beans::AstNode;
use napi_derive::napi;
use serde::Serialize;

#[napi(object)]
#[derive(Debug, Serialize, Clone)]
pub struct ModuleMemberUsageResponse {
  pub lib_name: String,
  pub member_name: String,
  pub file_path: String,
  pub ast_node: AstNode,
  pub props: Vec<JSXProps>,
}

#[napi(object, js_name = "JSXProps")]
#[derive(Debug, Serialize, Clone)]
pub struct JSXProps {
  pub namespace: Option<String>,
  pub name: String,
  pub value: Option<String>,
}
