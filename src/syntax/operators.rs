use serde::Deserialize;

use super::compat::{Compat, Status, Support};

#[derive(Debug, Deserialize)]
pub struct Operators {
  pub addition_assignment: Compat,

  pub addition: Compat,

  pub assignment: Compat,

  pub spread: Compat,

  pub spread_in_arrays: Compat,

  pub spread_in_function_calls: Compat,

  pub spread_in_object_literals: Compat,

  pub exponentiation_assignment: Compat,

  pub r#null: Compat,

  pub nullish_coalescing: Compat,

  pub nullish_coalescing_assignment: Compat,

  pub r#yield: Compat,

  pub r#yield_star: Compat,

  pub r#await: Compat,

  pub r#top_level: Compat,

  pub destructuring: Compat,

  pub rest_in_arrays: Compat,

  pub rest_in_objects: Compat,
}
