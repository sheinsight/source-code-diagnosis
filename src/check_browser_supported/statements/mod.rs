use super::visitor::SyntaxVisitor;

pub mod async_function;
pub mod async_generator_function;
pub mod block;
pub mod r#break;
pub mod class;
pub mod r#const;
pub mod r#continue;
pub mod debugger;
pub mod do_while;
pub mod empty;
pub mod export;
pub mod export_default;
pub mod export_namespace;
pub mod r#for;
pub mod for_await_of;
pub mod for_in;
pub mod for_of;
// pub mod for_of_async_iterators;
// pub mod for_of_closing_iterators;
pub mod function;
pub mod function_trailing_comma_in_parameters;
pub mod generator_function;
// pub mod generator_function_iterator_result_object;
// pub mod generator_function_not_constructable_with_new;
pub mod generator_function_trailing_comma_in_parameters;
pub mod if_else;
pub mod import;
pub mod import_import_assertions;
pub mod import_import_assertions_type_css;
pub mod import_import_assertions_type_json;
pub mod import_import_attributes;
pub mod import_import_attributes_type_css;
pub mod import_import_attributes_type_json;
pub mod label;
pub mod r#let;
pub mod r#return;
pub mod switch;
pub mod throw;
pub mod try_catch;
pub mod try_catch_optional_catch_binding;
pub mod var;
pub mod r#while;
pub mod with;

pub fn setup_statements(v: &mut SyntaxVisitor) {
  async_function::setup(v);
  async_generator_function::setup(v);
  // block::setup(v);
  // r#break::setup(v);
  class::setup(v);
  r#const::setup(v);
  // r#continue::setup(v);
  debugger::setup(v);
  // do_while::setup(v);
  // empty::setup(v);
  export_default::setup(v);
  export_namespace::setup(v);
  export::setup(v);
  for_await_of::setup(v);
  // for_in::setup(v);
  for_of::setup(v);
  // r#for::setup(v);
  function_trailing_comma_in_parameters::setup(v);
  // function::setup(v);
  generator_function_trailing_comma_in_parameters::setup(v);
  generator_function::setup(v);
  // if_else::setup(v);
  import_import_assertions_type_css::setup(v);
  import_import_assertions_type_json::setup(v);
  import_import_assertions::setup(v);
  import_import_attributes_type_css::setup(v);
  import_import_attributes_type_json::setup(v);
  import_import_attributes::setup(v);
  import::setup(v);
  // label::setup(v);
  r#let::setup(v);
  // r#return::setup(v);
  // switch::setup(v);
  // throw::setup(v);
  try_catch_optional_catch_binding::setup(v);
  // try_catch::setup(v);
  // var::setup(v);
  // r#while::setup(v);
  // with::setup(v);
}