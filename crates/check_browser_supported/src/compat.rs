use napi_derive::napi;
use oxc::semantic::{AstNode, AstNodes};
use oxc::span::GetSpan;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize,
)]
pub struct Compat {
  pub name: String,
  pub mdn_url: String,
  pub description: String,
  pub tags: Vec<String>,
  pub support: Support,
}

// #[napi(object)]
// #[derive(
//   Debug, Default, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize,
// )]
// pub struct Status {
//   pub experimental: bool,
//   pub standard_track: bool,
//   pub deprecated: bool,
// }

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
  pub compat: Compat,
  pub file_path: String,
  pub ast_node: beans::AstNode,
}

impl CompatBox {
  pub fn new(
    ast_node: beans::AstNode,
    compat: Compat,
    file_path: String,
  ) -> Self {
    Self {
      name: compat.name.clone(),
      ast_node,
      compat,
      file_path: file_path.to_string(),
    }
  }
}

pub trait CompatHandler: Send + Sync {
  fn handle<'a>(
    &self,
    source_code: &str,
    node: &AstNode<'a>,
    nodes: &AstNodes<'a>,
  ) -> bool;

  fn get_compat(&self) -> &Compat;
}

#[allow(dead_code)]
pub trait AstNodeHelper<'a> {
  fn text(&self, source: &str) -> String;
}

impl<'a> AstNodeHelper<'a> for AstNode<'a> {
  fn text(&self, source: &str) -> String {
    let span = self.kind().span();
    source[span.start as usize..span.end as usize].to_string()
  }
}

pub fn get_source_code_segment<'a, N: GetSpan>(
  source_code: &str,
  node: &N,
) -> String {
  let span = node.span();
  source_code[span.start as usize..span.end as usize].to_string()
}
