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
pub struct Response {
  pub path: String,
  pub errors: Vec<String>,
}

pub fn check_syntax(args: Args) -> anyhow::Result<Vec<Response>> {
  let glob = Glob::new(&args.pattern)?;

  let files = glob
    .walk(&args.cwd)
    .not(args.ignore.iter().map(|s| s.as_str()))?;

  Ok(
    files
      .par_bridge()
      .filter_map(|item| {
        let entry = item.ok()?;
        let path = entry.path();
        let path_string = path.to_string_lossy().to_string();

        match std::fs::read_to_string(path) {
          Err(err) => Some(Response {
            path: path_string,
            errors: vec![err.to_string()],
          }),
          Ok(content) => {
            let allocator = oxc_allocator::Allocator::default();
            let parser = oxc_parser::Parser::new(
              &allocator,
              &content,
              oxc_span::SourceType::default(),
            );

            let parse_response = parser.parse();

            (!parse_response.errors.is_empty()).then(|| Response {
              path: pathdiff::diff_paths(&path, &args.cwd)
                .unwrap()
                .to_string_lossy()
                .to_string(),
              errors: parse_response
                .errors
                .iter()
                .map(|e| e.to_string())
                .collect(),
            })
          }
        }
      })
      .collect(),
  )
}
