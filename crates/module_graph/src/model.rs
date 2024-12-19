use std::collections::HashMap;

use camino::Utf8PathBuf;
use napi_derive::napi;
use serde::Serialize;

#[derive(Debug)]
pub enum ConversionError {
  None,
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct JsArgs {
  pub cwd: String,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub resolve: Option<JsResolve>,
}

impl Default for JsArgs {
  fn default() -> Self {
    Self {
      cwd: ".".into(),
      pattern: Some("**/*.{js,ts,jsx,tsx}".to_owned()),
      ignore: Some(vec![
        "**/node_modules/**".to_owned(),
        "**/*.d.ts".to_owned(),
      ]),
      resolve: Some(JsResolve::default()),
    }
  }
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct JsResolve {
  pub extensions: Option<Vec<String>>,
  pub alias: Option<HashMap<String, Vec<String>>>,
  pub modules: Option<Vec<String>>,
}

impl Default for JsResolve {
  fn default() -> Self {
    Self {
      extensions: Some(vec![
        ".js".into(),
        ".jsx".into(),
        ".ts".into(),
        ".tsx".into(),
        ".json".into(),
        ".node".into(),
        ".css".into(),
        ".scss".into(),
        ".less".into(),
        ".d.ts".into(),
      ]),
      alias: Some(HashMap::new()),
      modules: Some(vec!["node_modules".into()]),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Args {
  pub cwd: String,
  pub pattern: String,
  pub ignore: Vec<String>,
  pub resolve: Resolve,
}

impl TryFrom<JsArgs> for Args {
  type Error = ConversionError;

  fn try_from(args: JsArgs) -> Result<Self, Self::Error> {
    let default = JsArgs::default();

    let cwd = Utf8PathBuf::from(&args.cwd).join("").to_string();
    let pattern = args
      .pattern
      .or(default.pattern)
      .ok_or(ConversionError::None)?;

    let ignore = args
      .ignore
      .or(default.ignore)
      .ok_or(ConversionError::None)?;

    let resolve = args
      .resolve
      .or(default.resolve)
      .ok_or(ConversionError::None)?;

    Ok(Self {
      cwd,
      pattern,
      ignore,
      resolve: resolve.try_into()?,
    })
  }
}

#[derive(Debug, Clone)]
pub struct Resolve {
  pub extensions: Vec<String>,
  pub alias: HashMap<String, Vec<String>>,
  pub modules: Vec<String>,
}

impl TryFrom<JsResolve> for Resolve {
  type Error = ConversionError;

  fn try_from(args: JsResolve) -> Result<Self, Self::Error> {
    let default = JsResolve::default();
    Ok(Self {
      extensions: args
        .extensions
        .or(default.extensions)
        .ok_or(ConversionError::None)?,
      alias: args.alias.or(default.alias).ok_or(ConversionError::None)?,
      modules: args
        .modules
        .or(default.modules)
        .ok_or(ConversionError::None)?,
    })
  }
}

/// edge
#[napi(object)]
#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone, PartialOrd, Ord)]
pub struct Edge {
  /// source file id
  /// please get value from `Graphics.dictionaries`
  pub source_id: String,
  /// target file id
  /// please get value from `Graphics.dictionaries`
  pub target_id: String,
  /// file collapse , not found the file ,
  /// - may be is unexpected import
  /// - this is node_modules lib , but node_modules is not installed
  pub missing: bool,
  /// target metadata
  pub target_metadata: Option<TargetMetadata>,
  /// ast node
  pub ast_node: beans::AstNode,
}

/// Metadata for a target module
#[napi(object)]
#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone, PartialOrd, Ord)]
pub struct TargetMetadata {
  /// module id
  /// please get value from `Graphics.dictionaries`
  pub module_id: String,
  /// may be
  /// e.g. node_modules/antd/lib/Button
  /// e.g. @babel/core/lib/something
  /// e.g. lodash/cloneDeep
  /// e.g. @/src/index.ts
  pub may_be: bool,
}

#[derive(Debug, Serialize)]
#[napi(object)]
pub struct GroupGraphics {
  /// dictionaries
  /// key is file id
  /// value is file path
  pub dictionaries: HashMap<String, String>,
  /// graph
  /// e.g. [ [Edge, Edge], [Edge, Edge] ]
  pub graph: Vec<Vec<Edge>>,
  /// invalid syntax files
  pub syntax_errors: Vec<String>,
}

#[napi(object)]
pub struct Graphics {
  pub dictionaries: HashMap<String, String>,
  pub graph: Vec<Edge>,
  pub syntax_errors: Vec<String>,
}
