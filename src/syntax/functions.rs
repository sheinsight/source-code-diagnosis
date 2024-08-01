use serde::Deserialize;

use super::compat::Compat;

#[derive(Debug, Deserialize)]
pub struct Functions {
  pub functions: Compat,

  pub arguments: Compat,

  pub arrow_functions: Compat,

  pub arrow_functions_trailing_comma: Compat,

  pub default_parameters: Compat,

  pub getter: Compat,

  pub getter_computed_property_names: Compat,

  pub method_definitions: Compat,

  pub method_definitions_async_generator_methods: Compat,

  pub method_definitions_async_methods: Compat,

  pub method_definitions_generator_methods_not_constructable: Compat,
}
