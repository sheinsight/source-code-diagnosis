use serde::Deserialize;

use super::compat::Compat;

#[derive(Debug, Deserialize)]
pub struct Operators {
  pub addition_assignment: Compat,

  pub addition: Compat,

  pub assignment: Compat,

  pub spread: Compat,

  pub spread_in_arrays: Compat,

  pub spread_in_function_calls: Compat,

  pub spread_in_object_literals: Compat,

  pub exponentiation: Compat,

  pub exponentiation_assignment: Compat,

  pub r#null: Compat,

  pub nullish_coalescing: Compat,

  pub nullish_coalescing_assignment: Compat,

  pub r#yield: Compat,

  pub r#yield_star: Compat,

  pub r#await: Compat,

  pub r#await_top_level: Compat,

  pub destructuring: Compat,

  pub computed_property_names: Compat,

  pub rest_in_arrays: Compat,

  pub rest_in_objects: Compat,

  pub import: Compat,

  pub options_parameter: Compat,

  pub import_meta: Compat,

  pub resolve: Compat,
}