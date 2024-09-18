use napi::Result;
use napi_derive::napi;
use utils::GlobOptions;

#[napi]
pub fn check_danger_strings(
  danger_strings: Vec<String>,
  options: Option<GlobOptions>,
) -> Result<Vec<check_danger_string::Response>> {
  check_danger_string::check_danger_strings(danger_strings, options)
}

#[napi]
pub fn check_module_member_usage(
  npm_name_vec: Vec<String>,
  options: Option<GlobOptions>,
) -> Result<Vec<module_member_usage::Response>> {
  module_member_usage::check_module_member_usage(npm_name_vec, options)
}

#[napi]
pub fn check_browser_supported(
  target: String,
  options: Option<utils::GlobOptions>,
) -> Result<Vec<check_browser_supported::CompatBox>> {
  check_browser_supported::check_browser_supported(target, options)
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
}
