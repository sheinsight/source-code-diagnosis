use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy,
)]
pub struct Span {
  pub start: u32,
  pub end: u32,
}
