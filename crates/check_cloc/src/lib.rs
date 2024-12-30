use napi_derive::napi;
use serde::Serialize;
use tokei::{Config, Languages};
use utils::GlobArgs;

#[napi(object)]
#[derive(Debug, Serialize)]
pub struct CheckCLOCResponse {
  pub language: String,
  pub lines_of_code: String,
}

pub fn check_cloc(args: GlobArgs) -> anyhow::Result<Vec<CheckCLOCResponse>> {
  let config = Config::default();
  let mut languages = Languages::new();

  let ignore_vec = args.ignore.iter().map(|s| s.as_str()).collect::<Vec<_>>();

  let paths = &[args.cwd.as_str()];

  languages.get_statistics(paths, &ignore_vec, &config);

  let responses = languages
    .into_iter()
    .map(|(lang, stats)| CheckCLOCResponse {
      language: lang.to_string(),
      lines_of_code: stats.code.to_string(),
    })
    .collect::<Vec<_>>();

  Ok(responses)
}
