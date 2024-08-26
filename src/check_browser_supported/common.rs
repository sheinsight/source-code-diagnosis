use oxc_ast::AstKind;
use oxc_span::Span;

use super::compat::CompatBox;

#[derive(Debug)]
pub struct Context<'a> {
  pub source_code: String,
  pub usage: Vec<CompatBox>,
  pub stack: Vec<AstKind<'a>>,
  pub is_strict_mode: bool,
  pub file_path: String,
}

impl Context<'_> {
  pub fn new(source_code: &str, file_path: &str) -> Self {
    Self {
      source_code: source_code.to_string(),
      usage: Vec::new(),
      stack: Vec::new(),
      is_strict_mode: false,
      file_path: file_path.to_string(),
    }
  }

  pub fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}
