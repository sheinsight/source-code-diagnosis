use std::{
  fs::read,
  path::PathBuf,
  sync::{Arc, Mutex},
};

use danger_string_location::DangerStringLocation;
use napi::{Error, Result};
use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;

use crate::oxc_visitor_processor::{oxc_visit_process, Options};

mod danger_string_location;

#[napi]
pub fn get_danger_strings_usage(
  danger_strings: Vec<String>,
  options: Option<Options>,
) -> Result<Vec<DangerStringLocation>> {
  let used = Arc::new(Mutex::new(Vec::new()));
  let x = {
    let used = Arc::clone(&used);
    move |path: PathBuf| {
      let source_text = read(&path)
        .map_err(|err| {
          Error::new(
            napi::Status::GenericFailure,
            format!("Failed to read file: {}: {}", path.display(), err),
          )
        })
        .unwrap();
      let source_text = String::from_utf8(source_text).unwrap();
      let source_type = SourceType::from_path(&path)
        .map_err(|e| Error::new(napi::Status::GenericFailure, e.0.to_string()))
        .unwrap();
      let allocator = Allocator::default();
      let ret = Parser::new(&allocator, &source_text, source_type).parse();
      let semantic = SemanticBuilder::new(&source_text, source_type)
        .build(&ret.program)
        .semantic;

      let mut inline_usages: Vec<DangerStringLocation> = Vec::new();

      for node in semantic.nodes().iter() {
        if let AstKind::StringLiteral(string_literal) = node.kind() {
          let value = string_literal.value.to_string();
          let span = string_literal.span;
          let start_position =
            crate::utils::offset_to_position(span.start as usize, &source_text)
              .unwrap();
          let end_position =
            crate::utils::offset_to_position(span.end as usize, &source_text)
              .unwrap();
          let loc = crate::utils::Location {
            start: start_position,
            end: end_position,
          };

          danger_strings
            .iter()
            .filter(|item| value.contains(&**item))
            .for_each(|item| {
              inline_usages.push(DangerStringLocation {
                raw_value: value.to_string(),
                match_danger_string: item.to_string(),
                start: span.start,
                end: span.end,
                file_path: path.display().to_string(),
                loc: loc.clone(),
              })
            })
        }
      }

      let mut used = used.lock().unwrap();
      used.extend(inline_usages);
    }
  };
  oxc_visit_process(x, options)?;

  let used = Arc::try_unwrap(used)
    .ok()
    .expect("Arc has more than one strong reference")
    .into_inner()
    .expect("Mutex cannot be locked");

  Ok(used)
}
