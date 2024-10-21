use std::collections::HashMap;

use camino::Utf8PathBuf;
use napi_derive::napi;
use serde::Serialize;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct DemoArgs {
  pub cwd: String,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub alias: Option<HashMap<String, Vec<String>>>,
  pub modules: Option<Vec<String>>,
}

impl Default for DemoArgs {
  fn default() -> Self {
    Self {
      cwd: "".to_string(),
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
#[napi(object)]
pub struct JsArgs {
  pub cwd: String,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub alias: Option<HashMap<String, Vec<String>>>,
  pub modules: Option<Vec<String>>,
}

impl JsArgs {
  pub fn get_cwd(&self) -> String {
    return Utf8PathBuf::from(self.cwd.clone()).join("").to_string();
  }

  pub fn get_pattern(&self) -> String {
    self
      .pattern
      .clone()
      .unwrap_or("**/*.{js,ts,jsx,tsx}".to_string())
  }

  pub fn get_ignore(&self) -> Vec<String> {
    let mut default_ignore = Vec::new();
    default_ignore.push("**/node_modules/**".to_string());
    default_ignore.push("**/*.d.ts".to_string());
    self.ignore.clone().unwrap_or(default_ignore)
  }

  pub fn get_alias(&self) -> HashMap<String, Vec<String>> {
    self.alias.clone().unwrap_or(HashMap::new())
  }

  pub fn get_modules(&self) -> Vec<String> {
    self
      .modules
      .clone()
      .unwrap_or(vec!["node_modules".to_string()])
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

#[derive(Debug, Serialize)]
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
