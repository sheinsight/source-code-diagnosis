use oxc_ast::Visit;

use super::location::Location;

pub struct DangerStringVisitor<'a> {
  pub used: &'a mut Vec<Location>,
  pub file_path: &'a String,
  pub danger_strings: &'a Vec<String>,
}

impl<'a> Visit<'a> for DangerStringVisitor<'a> {
  fn visit_string_literal(&mut self, lit: &oxc_ast::ast::StringLiteral<'a>) {
    let value = lit.value.to_string();

    self
      .danger_strings
      .iter()
      .filter(|item| value.contains(&**item))
      .for_each(|item| {
        self.used.push(Location {
          raw_value: value.to_string(),
          match_danger_string: item.to_string(),
          start: lit.span.start,
          end: lit.span.end,
          file_path: self.file_path.clone(),
        })
      });
  }
}
