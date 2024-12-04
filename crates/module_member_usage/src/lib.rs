use anyhow::Result;
use handler::ModuleMemberUsageHandler;
use utils::GlobArgs;
use utils::{glob_by, SemanticBuilder};

mod handler;
mod response;
pub use response::Response;

pub fn check_module_member_usage(
  npm_name_vec: Vec<String>,
  args: GlobArgs,
) -> Result<Vec<Response>> {
  let responses = glob_by(
    |path| {
      let builder = SemanticBuilder::with_file(&path).unwrap();

      let handler = match builder.build_handler() {
        Ok(handler) => handler,
        Err(e) => {
          eprintln!("parse error: {}", e);
          return None;
        }
      };

      let inline_usages =
        ModuleMemberUsageHandler::new(npm_name_vec.clone(), path, handler)
          .handle();

      Some(inline_usages)
    },
    args,
  )?
  .into_iter()
  .flatten()
  .collect();

  Ok(responses)
}
