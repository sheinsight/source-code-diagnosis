use napi_derive::napi;
use serde::{Deserialize, Serialize};

use crate::{Position, Span};

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy,
)]
pub struct Location {
  pub start: Position,
  pub end: Position,
}

impl Location {
  pub fn new(start: Position, end: Position) -> Self {
    Location { start, end }
  }

  pub fn with_source(source_text: &str, span: Span) -> Self {
    let start = Position::with_source(source_text, span.start as usize);
    let end = Position::with_source(source_text, span.end as usize);
    Self { start, end }
  }
}
