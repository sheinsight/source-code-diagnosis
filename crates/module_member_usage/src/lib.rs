use std::{
  path::PathBuf,
  sync::{Arc, Mutex},
};

use handler::ModuleMemberUsageHandler;
use napi::Result;
pub use response::Response;
use utils::{glob, GlobOptions, SemanticBuilder};
mod handler;
mod response;

pub fn check_module_member_usage(
  npm_name_vec: Vec<String>,
  options: Option<GlobOptions>,
) -> Result<Vec<Response>> {
  let used = Arc::new(Mutex::new(Vec::new()));
  let x = {
    let used = Arc::clone(&used);
    move |path: PathBuf| {
      let builder = SemanticBuilder::file(path.clone());
      let handler = builder.build_handler();

      let inline_usages = ModuleMemberUsageHandler::new(
        npm_name_vec.clone(),
        path.clone(),
        handler,
      )
      .handle()
      .unwrap();

      let mut used = used.lock().unwrap();
      used.extend(inline_usages);
    }
  };
  glob(x, options).map_err(|err| {
    napi::Error::new(napi::Status::GenericFailure, err.to_string())
  })?;

  let used = Arc::try_unwrap(used)
    .ok()
    .expect("Arc has more than one strong reference")
    .into_inner()
    .expect("Mutex cannot be locked");

  println!("{:?}", used.len());

  Ok(used)
}
