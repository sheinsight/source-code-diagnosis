use env_logger::Builder;
use napi::Result;
use napi_derive::napi;
use std::{
  str::FromStr,
  sync::atomic::{AtomicBool, Ordering},
};
use tap::Pipe;
use utils::GlobJsArgs;

static LOGGER_INITIALIZED: AtomicBool = AtomicBool::new(false);

fn to_napi_error<E: ToString>(e: E) -> napi::Error {
  napi::Error::new(napi::Status::GenericFailure, e.to_string())
}

#[napi]
pub fn enable_log(level: Option<String>) -> Result<()> {
  if LOGGER_INITIALIZED.load(Ordering::Relaxed) {
    return Ok(());
  }

  let level = level.unwrap_or_else(|| "info".to_string());

  Builder::new()
    .filter_level(
      log::LevelFilter::from_str(&level).unwrap_or(log::LevelFilter::Info),
    )
    .try_init()
    .map_err(|e| {
      napi::Error::new(napi::Status::GenericFailure, e.to_string())
    })?;

  LOGGER_INITIALIZED.store(true, Ordering::Relaxed);
  Ok(())
}

#[napi]
pub async fn get_graph(
  args: module_graph::model::JsArgs,
) -> Result<module_graph::model::Graphics> {
  TryInto::<module_graph::model::Args>::try_into(args)
    .map_err(|_| to_napi_error("Invalid arguments"))?
    .pipe(module_graph::edges::get_graph)
    .map_err(to_napi_error)
}

#[napi]
pub async fn check_cloc(
  args: utils::GlobJsArgs,
) -> Result<Vec<check_cloc::CheckCLOCResponse>> {
  check_cloc::check_cloc(args.into()).map_err(to_napi_error)
}

#[napi]
pub async fn check_cycle(
  args: module_graph::model::JsArgs,
) -> Result<module_graph::model::GroupGraphics> {
  TryInto::<module_graph::model::Args>::try_into(args)
    .map_err(|_| to_napi_error("Invalid arguments"))?
    .pipe(module_graph::edges::get_graph)
    .map_err(to_napi_error)?
    .pipe(module_graph::cycle::check_cycle)
    .map_err(to_napi_error)
}

#[napi]
pub async fn check_phantom_dependencies(
  dependencies: Vec<String>,
  args: module_graph::model::JsArgs,
) -> Result<module_graph::model::Graphics> {
  TryInto::<module_graph::model::Args>::try_into(args)
    .map_err(|_| to_napi_error("Invalid arguments"))?
    .pipe(module_graph::edges::get_graph)
    .map_err(to_napi_error)?
    .pipe(|graph| {
      module_graph::phantom_dependencies::check_phantom_dependencies(
        dependencies,
        graph,
      )
    })
    .map_err(to_napi_error)
}

#[napi]
pub async fn check_oxlint(
  oxlint_config: String,
  args: utils::GlobJsArgs,
) -> Result<Vec<check_oxlint::CheckOxlintResponse>> {
  check_oxlint::check_oxlint(oxlint_config, args.into()).map_err(to_napi_error)
}

#[napi]
pub async fn check_danger_jsx_props(
  danger_jsx_props: Vec<String>,
  args: utils::GlobJsArgs,
) -> Result<Vec<check_danger_jsx_props::CheckDangerJsxPropsResponse>> {
  check_danger_jsx_props::check_danger_jsx_props(danger_jsx_props, args.into())
    .map_err(to_napi_error)
}

#[napi]
pub async fn check_danger_strings(
  danger_strings: Vec<String>,
  args: utils::GlobJsArgs,
) -> Result<Vec<check_danger_string::CheckDangerResponse>> {
  check_danger_string::check_danger_strings(danger_strings, args.into())
    .map_err(to_napi_error)
}

#[napi]
pub async fn check_module_member_usage(
  npm_name_vec: Vec<String>,
  args: GlobJsArgs,
) -> Result<Vec<module_member_usage::response::ModuleMemberUsageResponse>> {
  module_member_usage::check_module_member_usage(npm_name_vec, args.into())
    .map_err(to_napi_error)
}

#[napi]
pub async fn check_filename_case(
  args: utils::GlobJsArgs,
) -> Result<Vec<check_filename_case::CheckFilenameCaseResponse>> {
  check_filename_case::check_filename_case(args.into()).map_err(to_napi_error)
}

#[napi]
pub async fn check_browser_supported(
  target: check_browser_supported::target::Target,
  args: utils::GlobJsArgs,
) -> Result<Vec<check_browser_supported::CompatBox>> {
  check_browser_supported::check_browser_supported(target, args.into())
    .map_err(to_napi_error)
}

#[napi]
pub async fn check_browser_supported_with_source_code(
  target: check_browser_supported::target::Target,
  source_code: String,
  file_path: String,
) -> Result<Vec<check_browser_supported::CompatBox>> {
  check_browser_supported::check_browser_supported_with_source_code(
    target,
    source_code,
    file_path,
  )
  .map_err(to_napi_error)
}

#[napi]
pub async fn check_syntax(
  args: utils::GlobJsArgs,
) -> Result<Vec<check_syntax::CheckSyntaxResponse>> {
  check_syntax::check_syntax(args.into()).map_err(to_napi_error)
}
