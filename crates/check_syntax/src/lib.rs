use napi_derive::napi;
use utils::{glob_by_semantic, GlobArgs, GlobErrorHandler, GlobSuccessHandler};

#[derive(Debug, Clone)]
#[napi(object, js_name = "CheckSyntaxResponse")]
pub struct CheckSyntaxResponse {
  pub path: String,
  pub errors: Vec<String>,
}

pub fn check_syntax(
  args: GlobArgs,
) -> anyhow::Result<Vec<CheckSyntaxResponse>> {
  let responses = glob_by_semantic(
    |GlobSuccessHandler { .. }| None,
    |GlobErrorHandler {
       relative_path,
       error,
       ..
     }| {
      Some(CheckSyntaxResponse {
        path: relative_path,
        errors: vec![error],
      })
    },
    &args,
  )?;

  Ok(responses)
}
