use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize,
)]
pub struct Compat {
  pub name: String,
  pub description: String,
  pub mdn_url: String,
  pub spec_url: Vec<String>,
  pub tags: Vec<String>,
  pub support: Support,
  pub status: Status,
}

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize,
)]
pub struct Status {
  pub experimental: bool,
  pub standard_track: bool,
  pub deprecated: bool,
}

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize,
)]
pub struct Support {
  pub chrome: String,
  pub chrome_android: String,
  pub firefox: String,
  pub firefox_android: String,
  pub safari: String,
  pub safari_ios: String,
  pub opera: String,
  pub opera_android: String,
  pub ie: String,
  pub edge: String,
  pub deno: String,
  pub nodejs: String,
}

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize,
)]
pub struct CompatBox {
  pub start: u32,
  pub end: u32,
  pub compat: Compat,
}
