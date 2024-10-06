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
  target: String,
  options: Option<utils::GlobOptions>,
) -> Result<Vec<check_browser_supported::CompatBox>> {
  check_browser_supported::check_browser_supported(target, options)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_browser_supported_with_source_code(
  target: String,
  source_code: String,
) -> Result<Vec<check_browser_supported::CompatBox>> {
  check_browser_supported::check_browser_supported_with_source_code(
    target,
    source_code,
  )
  .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_cycle(
  options: Option<module_graph::Options>,
) -> Result<Vec<Vec<module_graph::Cycle>>> {
  module_graph::check_cycle(options).map_err(|err| {
    napi::Error::new(napi::Status::GenericFailure, err.to_string())
  })
}

#[napi]
pub fn check_dependents(
  file: String,
  options: Option<module_graph::Options>,
) -> Result<module_graph::DependencyNode> {
  let _ = init_logger();
  module_graph::get_dependents(file, options)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_dependencies(
  file: String,
  options: Option<module_graph::Options>,
) -> Result<module_graph::DependencyNode> {
  let _ = init_logger();
  module_graph::get_dependencies(file, options)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn init_logger() -> napi::Result<()> {
  env_logger::Builder::from_env(Env::default().filter_or("SHINED_LOG", "info"))
    .try_init()
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}
