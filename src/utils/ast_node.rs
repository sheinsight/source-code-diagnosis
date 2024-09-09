use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy,
)]
pub struct AstNode {
  pub span: Span,
  pub loc: Location,
}

impl AstNode {
  pub fn new(span: (u32, u32), loc: Location) -> Self {
    Self {
      span: Span {
        start: span.0,
        end: span.1,
      },
      loc,
    }
  }
}

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy,
)]
pub struct Span {
  pub start: u32,
  pub end: u32,
}

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy,
)]
pub struct Location {
  pub start: Position,
  pub end: Position,
}

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy,
)]
pub struct Position {
  pub line: u32,
  pub col: u32,
}
