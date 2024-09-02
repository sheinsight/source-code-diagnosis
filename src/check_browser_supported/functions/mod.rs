use arguments::Arguments;
use arrow_functions::ArrowFunctions;
use arrow_functions_trailing_comma::ArrowFunctionsTrailingComma;
use callee::Callee;
use default_parameters_destructured_parameter_with_default_value_assignment::DefaultParametersDestructuredParameterWithDefaultValueAssignment;
use default_parameters_parameters_without_defaults_after_default_parameters::DefaultParametersParametersWithoutDefaultsAfterDefaultParameters;
use functions::FunctionsDeclarations;
use get::Get;
use get_computed_property_names::GetComputedPropertyNames;
use length::Length;
use method_definitions::MethodDefinitions;
use method_definitions_async_generator_methods::MethodDefinitionsAsyncGeneratorMethods;
use method_definitions_async_methods::MethodDefinitionsAsyncMethods;
use rest_parameters::RestParameters;
use rest_parameters_destructuring::RestParametersDestructuring;
use set::Set;
use set_computed_property_names::SetComputedPropertyNames;

use super::compat::CompatHandler;

pub mod arguments;
pub mod arrow_functions;
pub mod arrow_functions_trailing_comma;
pub mod callee;
pub mod default_parameters;
pub mod default_parameters_destructured_parameter_with_default_value_assignment;
pub mod default_parameters_parameters_without_defaults_after_default_parameters;
pub mod functions;
pub mod get;
pub mod get_computed_property_names;
pub mod iterator;
pub mod length;
pub mod method_definitions;
pub mod method_definitions_async_generator_methods;
pub mod method_definitions_async_methods;
pub mod method_definitions_generator_methods_not_constructable;
pub mod rest_parameters;
pub mod rest_parameters_destructuring;
pub mod set;
pub mod set_computed_property_names;

pub fn setup() -> Vec<Box<dyn CompatHandler>> {
  vec![
    Box::new(FunctionsDeclarations::default()),
    Box::new(Arguments::default()),
    Box::new(Callee::default()),
    Box::new(Length::default()),
    Box::new(iterator::Iterator::default()),
    Box::new(ArrowFunctions::default()),
    Box::new(ArrowFunctionsTrailingComma::default()),
    Box::new(
      DefaultParametersDestructuredParameterWithDefaultValueAssignment::default(
      ),
    ),
    Box::new(
      DefaultParametersParametersWithoutDefaultsAfterDefaultParameters::default(
      ),
    ),
    Box::new(GetComputedPropertyNames::default()),
    Box::new(Get::default()),
    Box::new(Set::default()),
    Box::new(SetComputedPropertyNames::default()),
    Box::new(RestParameters::default()),
    Box::new(RestParametersDestructuring::default()),
    Box::new(MethodDefinitions::default()),
    Box::new(MethodDefinitionsAsyncMethods::default()),
    Box::new(MethodDefinitionsAsyncGeneratorMethods::default()),
  ]
}
