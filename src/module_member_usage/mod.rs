use std::{
  path::PathBuf,
  sync::{Arc, Mutex},
};

use handler::ModuleMemberUsageHandler;
use napi::Result;
use response::Response;

use crate::{
  oxc_visitor_processor::{oxc_visit_process, Options},
  utils::semantic_builder::SemanticBuilder,
};

mod handler;
pub mod response;

#[napi]
pub fn get_module_member_usage(
  npm_name_vec: Vec<String>,
  options: Option<Options>,
) -> Result<Vec<Response>> {
  let used = Arc::new(Mutex::new(Vec::new()));
  let x = {
    let used = Arc::clone(&used);
    move |path: PathBuf| {
      let builder = SemanticBuilder::new(&path);
      let handler = builder.build_handler();

      let inline_usages =
        ModuleMemberUsageHandler::new(npm_name_vec.clone(), handler)
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
