use napi_derive::napi;
use serde::{Deserialize, Serialize};

use crate::{Location, Span};

#[napi(object, js_name = "AstNode")]
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
pub struct AstNode {
  pub span: Span,
  pub loc: Location,
}

impl AstNode {
  pub fn with_source_and_span(
    source_text: &str,
    span: &oxc::span::Span,
  ) -> Self {
    let span = Span::new(span);
    let loc = Location::with_source(source_text, span);
    Self { span, loc }
  }

  pub fn with_source_and_ast_node(
    source_text: &str,
    node: &oxc::semantic::AstNode,
  ) -> Self {
    let oxc_span = oxc::span::GetSpan::span(&node.kind());
    let span = Span::new(&oxc_span);
    let loc = Location::with_source(source_text, span);
    Self { span, loc }
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
