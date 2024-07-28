#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod danger_string_usage;
mod module_member_usage;
mod oxc_visitor_processor;
mod syntax;

pub use danger_string_usage::get_danger_strings_usage;
pub use module_member_usage::get_module_member_usage;
