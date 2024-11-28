use napi_derive::napi;
use rayon::prelude::*;
use wax::Glob;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct JsArgs {
  pub cwd: String,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Args {
  pub cwd: String,
  pub pattern: String,
  pub ignore: Vec<String>,
}

impl From<JsArgs> for Args {
  fn from(args: JsArgs) -> Self {
    let cwd = camino::Utf8PathBuf::from(&args.cwd).join("").to_string();
    let pattern = args.pattern.unwrap_or("**/*.{js,ts,jsx,tsx}".to_owned());
    let ignore = args.ignore.unwrap_or(vec![
      "**/node_modules/**".to_owned(),
      "**/*.d.ts".to_owned(),
      "node_modules/**".to_owned(),
    ]);
    Self {
      cwd,
      pattern,
      ignore,
    }
  }
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct CheckFilenameCaseResponse {
  pub path: String,
}

pub fn check_filename_case(
  args: Args,
) -> anyhow::Result<Vec<CheckFilenameCaseResponse>> {
  let glob = Glob::new(&args.pattern)?;

  let files = glob
    .walk(&args.cwd)
    .not(args.ignore.iter().map(|s| s.as_str()))?;

  let res = files
    .par_bridge()
    .filter_map(|item| {
      let entry = item.ok()?;
      let path = entry.path();

      let path = pathdiff::diff_paths(path, &args.cwd)?;

      let file_name = path.file_name()?.to_string_lossy();

      if file_name.to_ascii_lowercase().eq(&file_name) {
        None
      } else {
        Some(CheckFilenameCaseResponse {
          path: path.to_string_lossy().to_string(),
        })
      }
    })
    .collect();

  Ok(res)
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_check_filename_case() {
//     let args = Args::from(JsArgs {
//       pattern: None,
//       ignore: None,
//     });
//     let res = check_filename_case(args);
//     if let Ok(res) = res {
//       for item in res {
//         println!("{}", item.path);
//       }
//     }
//   }
// }
