use env_logger::Env;
use napi::Result;
use napi_derive::napi;
use utils::GlobOptions;

#[napi]
pub fn check_danger_strings(
  danger_strings: Vec<String>,
  options: Option<GlobOptions>,
) -> Result<Vec<check_danger_string::Response>> {
  check_danger_string::check_danger_strings(danger_strings, options)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_module_member_usage(
  npm_name_vec: Vec<String>,
  options: Option<GlobOptions>,
) -> Result<Vec<module_member_usage::Response>> {
  module_member_usage::check_module_member_usage(npm_name_vec, options)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_browser_supported(
  target: check_browser_supported::Target,
  options: Option<utils::GlobOptions>,
) -> Result<Vec<check_browser_supported::CompatBox>> {
  check_browser_supported::check_browser_supported(target, options)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_browser_supported_with_source_code(
  target: check_browser_supported::Target,
  source_code: String,
) -> Result<Vec<check_browser_supported::CompatBox>> {
  check_browser_supported::check_browser_supported_with_source_code(
    target,
    source_code,
  )
  .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn test(file: String, args: module_graph::model::JsArgs) {
  // let mut graph = module_graph::graph::Graph::new(Some(args.clone()));
  // if let Ok(cycles) = graph.check_cycle() {
  //   println!("--->>>  {:?}", &cycles.graph);
  //   println!("--->>>  {:?}", &cycles.dictionaries);
  // }

  let cwd = args.cwd;
  let ignore = args.ignore.unwrap_or_default();
  let pattern = args.pattern.unwrap_or_default();

  let mut graph = module_graph::graph::Graph::new(module_graph::model::Args {
    alias: args.alias.unwrap_or_default(),
    modules: args.modules.unwrap_or_default(),
    cwd: cwd.as_str(),
    ignore: ignore.iter().map(|s| s.as_str()).collect(),
    pattern: pattern.as_str(),
  });
  if let Ok(cycles) = graph.check_dependencies(file.to_string()) {
    println!("--->>>  {:?}", &cycles.graph);
    println!("--->>>  {:?}", &cycles.dictionaries);
  }
}

#[napi]
pub fn check_cycle(
  args: module_graph::model::JsArgs,
) -> Result<module_graph::model::GroupGraphics> {
  let cwd = args.get_cwd();

  let ignore = args.get_ignore();

  let pattern = args.get_pattern();

  let mut graph = module_graph::graph::Graph::new(module_graph::model::Args {
    alias: args.get_alias(),
    modules: args.get_modules(),
    ignore: ignore.iter().map(AsRef::as_ref).collect(),
    cwd: &cwd,
    pattern: &pattern,
  });
  let res = graph.check_cycle();
  res.map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_dependents(
  file: String,
  args: module_graph::model::JsArgs,
) -> Result<module_graph::model::Graphics> {
  let cwd = args.get_cwd();

  let ignore = args.get_ignore();

  let pattern = args.get_pattern();

  let mut graph = module_graph::graph::Graph::new(module_graph::model::Args {
    alias: args.get_alias(),
    modules: args.get_modules(),
    ignore: ignore.iter().map(AsRef::as_ref).collect(),
    cwd: &cwd,
    pattern: &pattern,
  });

  graph
    .check_dependents(file)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_dependencies(
  file: String,
  args: module_graph::model::JsArgs,
) -> Result<module_graph::model::Graphics> {
  let cwd = args.get_cwd();

  let ignore = args.get_ignore();

  let pattern = args.get_pattern();

  let mut graph = module_graph::graph::Graph::new(module_graph::model::Args {
    alias: args.get_alias(),
    modules: args.get_modules(),
    ignore: ignore.iter().map(AsRef::as_ref).collect(),
    cwd: &cwd,
    pattern: &pattern,
  });

  graph
    .check_dependencies(file)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn init_logger() -> napi::Result<()> {
  env_logger::Builder::from_env(Env::default().filter_or("SHINED_LOG", "info"))
    .try_init()
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}
