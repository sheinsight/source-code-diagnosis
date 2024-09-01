use oxc_semantic::{AstNode, AstNodes};
use oxc_span::GetSpan;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize,
)]
pub struct Compat {
  pub name: String,
  pub description: String,
  pub tags: Vec<String>,
  pub support: Support,
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
  pub chrome_android: String,
  pub firefox: String,
  pub firefox_android: String,
  pub safari: String,
  pub safari_ios: String,
  pub edge: String,
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
  pub compat: Compat,
  pub file_path: String,
}

impl CompatBox {
  pub fn new(span: oxc_span::Span, compat: Compat, file_path: String) -> Self {
    Self {
      name: compat.name.clone(),
      span: Span::from_oxc_span(span.clone()),
      compat: compat,
      file_path: file_path.to_string(),
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

#[napi]
pub enum CompatType {
  ClassConstructor,
  ClassExtends,
  ClassPrivateClassFieldsIn,
  ClassPrivateClassFields,
  ClassPrivateMethods,
  ClassPublicClassFields,
  ClassStaticClassFields,
  ClassStaticInitializationBlocks,
  ClassStatic,
  Function,
  Grammar,
  Operator,
  Statement,
}

impl Span {
  pub fn from_oxc_span(span: oxc_span::Span) -> Self {
    Self {
      start: span.start,
      end: span.end,
    }
  }
}

pub trait CompatHandler {
  fn handle<'a>(&self, node: &AstNode<'a>, nodes: &AstNodes<'a>) -> bool;

  fn get_compat(&self) -> &Compat;
}

pub trait AstNodeHelper<'a> {
  fn text(&self, source: &str) -> String;
}

impl<'a> AstNodeHelper<'a> for AstNode<'a> {
  fn text(&self, source: &str) -> String {
    let span = self.kind().span();
    source[span.start as usize..span.end as usize].to_string()
  }
}
