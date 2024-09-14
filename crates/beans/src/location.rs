use napi_derive::napi;
use serde::{Deserialize, Serialize};

use crate::Position;

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy,
)]
pub struct Location {
  pub start: Position,
  pub end: Position,
}
