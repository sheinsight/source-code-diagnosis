use serde::{Deserialize, Serialize};

use super::Position;

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize,
)]
pub struct Location {
  pub start: Position,
  pub end: Position,
}
