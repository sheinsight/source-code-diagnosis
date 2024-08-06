use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize,
)]
pub struct Compat {
  pub name: String,
  pub description: String,
  // pub mdn_url: String,
  // pub spec_url: Vec<String>,
  pub tags: Vec<String>,
  pub support: Support,
  // pub status: Status,
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
  pub firefox: String,
  pub opera: String,
  pub edge: String,
  pub safari: String,
  pub node: String,
  pub deno: String,
}

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize,
)]
pub struct CompatBox {
  pub name: String,
  pub span: Span,
  // pub code_seg: String,
  pub compat: Compat,
}

impl CompatBox {
  pub fn new(span: &oxc_span::Span, compat: &Compat) -> Self {
    Self {
      name: compat.name.clone(),
      span: Span::from_oxc_span(span.clone()),
      compat: compat.clone(),
    }
  }
}

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize,
)]
pub struct Span {
  pub start: u32,
  pub end: u32,
}

impl Span {
  pub fn from_oxc_span(span: oxc_span::Span) -> Self {
    Self {
      start: span.start,
      end: span.end,
    }
  }
}
