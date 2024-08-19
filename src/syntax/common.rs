use oxc_ast::AstKind;

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
