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
  arguments_callee::setup_arguments_callee(v);
  arguments_iterator::setup_arguments_iterator(v);
  arguments_length::setup_arguments_length(v);
  arrow_functions_trailing_comma::setup_arrow_functions_trailing_comma(v);
  arrow_functions::setup_arrow_functions(v);
  block_level_functions::setup_block_level_functions(v);
  default_parameters_destructured_parameter_with_default_value_assignment::setup_default_parameters_destructured_parameter_with_default_value_assignment(v);
  default_parameters_parameters_without_defaults_after_default_parameters::setup_default_parameters_parameters_without_defaults_after_default_parameters(v);
  default_parameters::setup_default_parameters(v);
  functions::setup_functions(v);
  get_computed_property_names::setup_get_computed_property_names(v);
  get::setup_get(v);
  method_definitions_async_generator_methods::setup_method_definitions_async_generator_methods(v);
  method_definitions_async_methods::setup_method_definitions_async_methods(v);
  method_definitions_generator_methods_not_constructable::setup_setup_method_definitions_generator_methods_not_constructable(v);
  method_definitions::setup_method_definitions(v);
  rest_parameters_destructuring::setup_rest_parameters_destructuring(v);
  rest_parameters::setup_rest_parameters(v);
  set_computed_property_names::setup_set_computed_property_names(v);
  set::setup_set(v);
}
