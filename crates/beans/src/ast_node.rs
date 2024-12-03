use napi_derive::napi;
use serde::{Deserialize, Serialize};

use crate::{Location, Span};

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy,
)]
pub struct AstNode {
  pub span: Span,
  pub loc: Location,
}

impl AstNode {
  pub fn with_source(source_text: &str, span: (u32, u32)) -> Self {
    let loc = Location::with_source(source_text, span);
    Self {
      span: Span {
        start: span.0,
        end: span.1,
      },
      loc,
    }
  }

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
