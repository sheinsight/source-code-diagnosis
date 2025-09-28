use anyhow::Result;
use process::process;
use response::ModuleMemberUsageResponse;
use utils::{glob_by_semantic, GlobArgs};
use utils::{GlobErrorHandler, GlobSuccessHandler};

pub mod r#const;
pub mod import_declaration_specifier_expand;
pub mod member_expression;
pub mod process;
pub mod response;

pub fn check_module_member_usage(
  npm_name_vec: Vec<String>,
  args: GlobArgs,
) -> Result<Vec<ModuleMemberUsageResponse>> {
  let responses = glob_by_semantic(
    |GlobSuccessHandler {
       semantic,
       relative_path,
       ..
     }| {
      let responses = process(&semantic, &npm_name_vec);
      Some(ModuleMemberUsageResponse {
        file_path: relative_path.clone(),
        items: responses,
        errors: vec![],
      })
    },
    |GlobErrorHandler { relative_path, .. }| {
      Some(ModuleMemberUsageResponse {
        file_path: relative_path.clone(),
        items: vec![],
        errors: vec![],
      })
    },
    &args,
  )?;

  Ok(responses)
}
