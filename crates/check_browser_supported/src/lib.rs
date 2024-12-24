mod classes;
mod compat;
mod functions;
mod grammar;
mod macros;
mod operators;
mod statements;

pub use compat::{CompatBox, CompatHandler};

use compat_checker::CompatChecker;
use target::Target;

mod browser_versions;
mod compat_checker;
pub mod target;

pub fn check_browser_supported_with_source_code(
  target: Target,
  source_code: String,
  file_path: String,
) -> anyhow::Result<Vec<CompatBox>> {
  let checker = CompatChecker::new(target)?;
  checker.check_source(&source_code, &file_path)
}

pub fn check_browser_supported(
  target: Target,
  args: utils::GlobArgs,
) -> anyhow::Result<Vec<CompatBox>> {
  let checker = CompatChecker::new(target)?;
  checker.check_glob(args)
}
