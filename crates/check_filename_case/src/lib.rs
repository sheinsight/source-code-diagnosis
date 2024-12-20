use std::collections::HashSet;

use napi_derive::napi;
use utils::{glob_by_semantic, GlobErrorHandler, GlobSuccessHandler};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[napi(object)]
pub struct CheckFilenameCaseResponse {
  pub path: String,
}

pub fn check_filename_case(
  args: utils::GlobArgs,
) -> anyhow::Result<Vec<CheckFilenameCaseResponse>> {
  let res = glob_by_semantic(
    |GlobSuccessHandler { relative_path, .. }| {
      let mut p = vec![];
      let path_str = relative_path.split("/").collect::<Vec<&str>>();
      let len = path_str.len();
      for (index, part) in path_str.into_iter().enumerate() {
        p.push(part);
        if !part.to_ascii_lowercase().eq(part) {
          break;
        }
        if index == len - 1 {
          p.clear();
        }
      }

      if !p.is_empty() {
        return Some(CheckFilenameCaseResponse { path: p.join("/") });
      }

      return None;
    },
    |GlobErrorHandler { .. }| None,
    &args,
  )?;

  let unique: Vec<_> = HashSet::<_>::from_iter(res).into_iter().collect();

  Ok(unique)
}
