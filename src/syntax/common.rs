use oxc_ast::AstKind;
use oxc_span::Span;

use super::compat::CompatBox;

pub trait CommonTrait {
  fn get_usage(&self) -> Vec<CompatBox>;
}

#[derive(Debug)]
pub struct Context<'a> {
  pub source_code: String,
  pub usage: Vec<CompatBox>,
  pub stack: Vec<AstKind<'a>>,
}

impl Context<'_> {
  pub fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}
