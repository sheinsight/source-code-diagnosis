#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod danger_string;
mod oxc_visit_processor;

pub use danger_string::get_usage_of_danger_strings;

#[napi]
pub fn a(a: i32, b: i32) -> i32 {
  a + b
}
