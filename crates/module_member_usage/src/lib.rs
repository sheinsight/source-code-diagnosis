use std::fs::read_to_string;
use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use anyhow::Result;
use handler::ModuleMemberUsageHandler;
use oxc_span::SourceType;
use parking_lot::Mutex;
use utils::{glob, GlobOptions, SemanticBuilder};

mod handler;
mod response;
pub use response::Response;

pub fn check_module_member_usage(
  npm_name_vec: Vec<String>,
  options: Option<GlobOptions>,
) -> Result<Vec<Response>> {
  let used = Arc::new(Mutex::new(Vec::new()));
  let x = {
    let used = Arc::clone(&used);
    move |path: PathBuf| {
      let source_code = read_to_string(&path).unwrap();

      let source_type = SourceType::from_path(&path).unwrap();

      let builder = SemanticBuilder::code(&source_code, source_type);
      let handler = builder.build_handler();

      let inline_usages = ModuleMemberUsageHandler::new(
        npm_name_vec.clone(),
        path.clone(),
        handler,
      )
      .handle();

      used.lock().extend(inline_usages);
    }
  };
  glob(x, options)?;

  let used = Arc::try_unwrap(used)
    .ok()
    .context("Arc has more than one strong reference")?
    .into_inner();

  Ok(used)
}
