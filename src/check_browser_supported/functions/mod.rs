use super::visitor::SyntaxVisitor;

pub mod arguments_callee;
pub mod arguments_iterator;
pub mod arguments_length;
pub mod arrow_functions;
pub mod arrow_functions_trailing_comma;
pub mod block_level_functions;
pub mod default_parameters;
pub mod default_parameters_destructured_parameter_with_default_value_assignment;
pub mod default_parameters_parameters_without_defaults_after_default_parameters;
pub mod functions;
pub mod get;
pub mod get_computed_property_names;
pub mod method_definitions;
pub mod method_definitions_async_generator_methods;
pub mod method_definitions_async_methods;
pub mod method_definitions_generator_methods_not_constructable;
pub mod rest_parameters;
pub mod rest_parameters_destructuring;
pub mod set;
pub mod set_computed_property_names;

pub fn setup_functions(v: &mut SyntaxVisitor) {
  arguments_callee::setup(v);
  arguments_iterator::setup(v);
  arguments_length::setup(v);
  arrow_functions_trailing_comma::setup(v);
  arrow_functions::setup(v);
  block_level_functions::setup(v);
  default_parameters_destructured_parameter_with_default_value_assignment::setup(v);
  default_parameters_parameters_without_defaults_after_default_parameters::setup(v);
  default_parameters::setup(v);
  functions::setup(v);
  get_computed_property_names::setup(v);
  get::setup(v);
  method_definitions_async_generator_methods::setup(v);
  method_definitions_async_methods::setup(v);
  method_definitions_generator_methods_not_constructable::setup(v);
  method_definitions::setup(v);
  rest_parameters_destructuring::setup(v);
  rest_parameters::setup(v);
  set_computed_property_names::setup(v);
  set::setup(v);
}
