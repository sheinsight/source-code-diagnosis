use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy,
)]
pub struct Position {
  pub line: u32,
  pub col: u32,
}
