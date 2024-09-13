#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod check_browser_supported;
mod danger_string_usage;
mod module_member_usage;
mod utils;

pub use check_browser_supported::check_browser_supported;
pub use check_browser_supported::check_browser_supported_with_source_code;
pub use danger_string_usage::get_danger_strings_usage;
pub use module_member_usage::get_module_member_usage;
