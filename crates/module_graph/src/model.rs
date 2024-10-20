use std::{collections::HashMap, env::current_dir};

use napi_derive::napi;
use serde::Serialize;

#[napi(object)]
pub struct JsArgs {
  pub cwd: Option<String>,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub alias: Option<HashMap<String, Vec<String>>>,
  pub modules: Option<Vec<String>>,
}

impl Default for JsArgs {
  fn default() -> Self {
    Self {
      cwd: Some(current_dir().unwrap().display().to_string()),
      pattern: Some("**/*.{js,ts,jsx,tsx}".to_string()),
      ignore: Some(vec![
        "**/node_modules/**".to_string(),
        "**/*.d.ts".to_string(),
      ]),
      alias: Some(HashMap::new()),
      modules: Some(vec!["node_modules".to_string()]),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Args<'a> {
  pub cwd: &'a str,
  pub pattern: &'a str,
  pub ignore: Vec<&'a str>,
  pub alias: HashMap<String, Vec<String>>,
  pub modules: Vec<String>,
}

#[napi(object)]
#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone)]
pub struct Edge {
  pub source: String,
  pub target: String,
  pub ast_node: beans::AstNode,
}

#[napi(object)]
pub struct GroupGraphics {
  pub dictionaries: HashMap<String, String>,
  pub graph: Vec<Vec<Edge>>,
}

#[napi(object)]
pub struct Graphics {
  pub dictionaries: HashMap<String, String>,
  pub graph: Vec<Edge>,
}
