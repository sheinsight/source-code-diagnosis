use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object, js_name = "Span")]
#[derive(
  Debug,
  Default,
  PartialEq,
  Eq,
  Hash,
  Clone,
  Serialize,
  Deserialize,
  Copy,
  PartialOrd,
  Ord,
)]
pub struct Span {
  pub start: u32,
  pub end: u32,
}

impl Span {
  pub fn new(span: &oxc_span::Span) -> Self {
    Self {
      start: span.start,
      end: span.end,
    }
  }
}
