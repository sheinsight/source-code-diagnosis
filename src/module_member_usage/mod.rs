use std::{
  collections::HashMap,
  fs::read_to_string,
  path::PathBuf,
  sync::{Arc, Mutex},
};

use handler::SemanticContext;
use module_member_usage_location::ModuleMemberUsageLocation;
use napi::{Error, Result};
use oxc_span::SourceType;

use crate::oxc_visitor_processor::{oxc_visit_process, Options};

mod handler;
pub mod module_member_usage_location;

#[napi]
pub fn get_module_member_usage(
  npm_name_vec: Vec<String>,
  options: Option<Options>,
) -> Result<Vec<ModuleMemberUsageLocation>> {
  let used = Arc::new(Mutex::new(Vec::new()));
  let x = {
    let used = Arc::clone(&used);
    move |path: PathBuf| {
      let inline_usages = SemanticContext::new(path)
        .build_handler(npm_name_vec.clone())
        .handle()
        .unwrap();

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

  println!("{:?}", used.len());

  Ok(used)
}
