use std::collections::HashSet;

use napi_derive::napi;
use utils::glob_by;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[napi(object)]
pub struct CheckFilenameCaseResponse {
  pub path: String,
}

pub fn check_filename_case(
  args: utils::GlobArgs,
) -> anyhow::Result<Vec<CheckFilenameCaseResponse>> {
  let res = glob_by(
    |path| {
      let path = pathdiff::diff_paths(path, &args.cwd)?;

      let str = path.display().to_string();

      let mut p = vec![];
      let path_str = str.split("/").collect::<Vec<&str>>();
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
    &args,
  )?;

  let unique: Vec<_> = HashSet::<_>::from_iter(res).into_iter().collect();

  Ok(unique)
}
