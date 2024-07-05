use serde::Serialize;

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct ModuleMemberUsageLocation {
  pub lib_name: String,
  pub member_name: String,
  pub start: u32,
  pub end: u32,
  pub file_path: String,
}
