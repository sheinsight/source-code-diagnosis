use std::{marker::PhantomData, path::PathBuf};

use oxc_ast::Visit;

use super::danger_string_location::DangerStringLocation;

#[derive(Debug)]
pub struct DangerStringVisitor<'a> {
  pub used: Vec<DangerStringLocation>,
  pub file_path: PathBuf,
  pub danger_strings: Vec<String>,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> DangerStringVisitor<'a> {
  pub fn new(file_path: PathBuf, danger_strings: Vec<String>) -> Self {
    Self {
      used: Vec::new(),
      file_path: file_path,
      danger_strings: danger_strings,
      _phantom: PhantomData {},
    }
  }
}

impl<'a> Visit<'a> for DangerStringVisitor<'a> {
  fn visit_string_literal(&mut self, lit: &oxc_ast::ast::StringLiteral<'a>) {
    let value = lit.value.to_string();

    self
      .danger_strings
      .iter()
      .filter(|item| value.contains(&**item))
      .for_each(|item| {
        self.used.push(DangerStringLocation {
          raw_value: value.to_string(),
          match_danger_string: item.to_string(),
          start: lit.span.start,
          end: lit.span.end,
          file_path: self.file_path.display().to_string(),
        })
      });
  }
}
