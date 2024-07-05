use serde::Serialize;

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct DangerStringLocation {
  pub raw_value: String,
  pub match_danger_string: String,
  pub start: u32,
  pub end: u32,
  pub file_path: String,
}
