use napi_derive::napi;
use utils::{glob_by_path, GlobArgs};

#[derive(Debug, Clone)]
#[napi(object, js_name = "CheckSyntaxResponse")]
pub struct CheckSyntaxResponse {
  pub path: String,
  pub errors: Vec<String>,
}

pub fn check_syntax(
  args: GlobArgs,
) -> anyhow::Result<Vec<CheckSyntaxResponse>> {
  let responses = glob_by_path(
    |path| {
      let builder = utils::SemanticBuilder::with_file(path);

      if builder.is_err() {
        return Some(CheckSyntaxResponse {
          path: path.display().to_string(),
          errors: vec![
            "File is not a valid JavaScript file, Please check the file syntax"
              .to_owned(),
          ],
        });
      }

      let builder = builder.unwrap();

      let semantic = builder.build();

      if semantic.is_err() {
        return Some(CheckSyntaxResponse {
          path: path.display().to_string(),
          errors: vec![
            "File is not a valid JavaScript file, Please check the file syntax"
              .to_owned(),
          ],
        });
      }

      None
    },
    &args,
  )?;

  Ok(responses)
}
