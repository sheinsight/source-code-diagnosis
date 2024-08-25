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
  pub is_strict_mode: bool,
}

impl Context<'_> {
  pub fn new(source_code: &str) -> Self {
    Self {
      source_code: source_code.to_string(),
      usage: Vec::new(),
      stack: Vec::new(),
      is_strict_mode: false,
    }
  }

  pub fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}
