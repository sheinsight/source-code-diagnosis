use beans::AstNode;
use napi_derive::napi;
use serde::Serialize;

#[napi(object)]
#[derive(Debug, Serialize, Clone)]
pub struct ModuleMemberUsageResponse {
  pub file_path: String,
  pub items: Vec<ModuleMemberUsageResponseItem>,
  pub errors: Vec<String>,
}

#[napi(object, js_name = "ModuleMemberUsageResponseItem")]
#[derive(Debug, Serialize, Clone)]
pub struct ModuleMemberUsageResponseItem {
  pub lib_name: String,
  pub module_name: String,
  pub member_name: String,
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
