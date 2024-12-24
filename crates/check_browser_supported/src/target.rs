use napi_derive::napi;

#[derive(Debug, Clone)]
#[napi[object]]
pub struct Target {
  pub chrome: String,
  pub firefox: Option<String>,
  pub safari: Option<String>,
  pub edge: Option<String>,
  pub node: Option<String>,
  // pub deno: Option<String>,
}
