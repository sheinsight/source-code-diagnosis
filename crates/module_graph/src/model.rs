use std::collections::HashMap;

use camino::Utf8PathBuf;
use napi_derive::napi;
use serde::Serialize;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct JsArgs {
  pub cwd: String,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub alias: Option<HashMap<String, Vec<String>>>,
  pub modules: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Args {
  pub cwd: String,
  pub pattern: String,
  pub ignore: Vec<String>,
  pub alias: HashMap<String, Vec<String>>,
  pub modules: Vec<String>,
}

impl From<JsArgs> for Args {
  fn from(args: JsArgs) -> Self {
    let cwd = Utf8PathBuf::from(&args.cwd).join("").to_string();
    let pattern = args.pattern.unwrap_or("**/*.{js,ts,jsx,tsx}".to_owned());
    let ignore = args.ignore.unwrap_or(vec![
      "**/node_modules/**".to_owned(),
      "**/*.d.ts".to_owned(),
    ]);
    let alias = args.alias.unwrap_or(HashMap::new());
    let modules = args.modules.unwrap_or(vec!["node_modules".to_owned()]);
    Self {
      cwd,
      pattern,
      ignore,
      alias,
      modules,
    }
  }
}

#[napi(object)]
#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone)]
pub struct Edge {
  pub source: String,
  pub target: String,
  pub ast_node: beans::AstNode,
}

#[derive(Debug, Serialize)]
#[napi(object)]
pub struct GroupGraphics {
  pub dictionaries: HashMap<String, String>,
  pub graph: Vec<Vec<Edge>>,
  pub invalid_syntax_files: Vec<String>,
}

#[napi(object)]
pub struct Graphics {
  pub dictionaries: HashMap<String, String>,
  pub graph: Vec<Edge>,
  pub invalid_syntax_files: Vec<String>,
}
