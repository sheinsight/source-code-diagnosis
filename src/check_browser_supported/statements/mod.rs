use super::compat::CompatHandler;

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
pub mod for_of_async_iterators;
pub mod for_of_closing_iterators;
pub mod function;
pub mod function_trailing_comma_in_parameters;
pub mod generator_function;
pub mod generator_function_iterator_result_object;
pub mod generator_function_not_constructable_with_new;
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
pub fn setup() -> Vec<Box<dyn CompatHandler>> {
  vec![
    Box::new(async_function::AsyncFunction::default()),
    Box::new(async_generator_function::AsyncGeneratorFunction::default()),
    Box::new(block::BlockStatement::default()),
    Box::new(r#break::BreakStatement::default()),
    Box::new(class::ClassesDeclarations::default()),
    Box::new(r#const::Const::default()),
    Box::new(r#continue::ContinueStatement::default()),
    Box::new(debugger::DebuggerStatement::default()),
    Box::new(do_while::DoWhileStatement::default()),
    Box::new(empty::EmptyStatement::default()),
    Box::new(export_default::ExportDefault::default()),
    Box::new(export_namespace::ExportNamespace::default()),
    Box::new(export::Export::default()),
    Box::new(for_await_of::ForAwaitOf::default()),
    Box::new(for_in::ForIn::default()),
    Box::new(for_of_async_iterators::ForOfAsyncIterators::default()),
    Box::new(for_of_closing_iterators::ForOfClosingIterators::default()),
    Box::new(for_of::ForOf::default()),
    Box::new(r#for::For::default()),
    Box::new(function::FunctionsDeclarations::default()),
    Box::new(function_trailing_comma_in_parameters::FunctionTrailingCommaInParameters::default()),
    Box::new(generator_function_iterator_result_object::GeneratorFunctionIteratorResultObject::default()),
    Box::new(generator_function_not_constructable_with_new::GeneratorFunctionNotConstructableWithNew::default()),
    Box::new(generator_function_trailing_comma_in_parameters::GeneratorFunctionTrailingCommaInParameters::default()),
    Box::new(generator_function::GeneratorFunction::default()),
    Box::new(if_else::IfElse::default()),
    Box::new(import_import_assertions_type_css::ImportImportAssertionsTypeCSS::default()),
    Box::new(import_import_assertions_type_json::ImportImportAssertionsTypeJson::default()),
    Box::new(import_import_assertions::ImportImportAssertions::default()),
    Box::new(import_import_attributes_type_css::ImportImportAttributesTypeCSS::default()),
    Box::new(import_import_attributes_type_json::ImportImportAttributesTypeJson::default()),
    Box::new(import_import_attributes::ImportImportAttributes::default()),
    Box::new(import::Import::default()),
    Box::new(label::LabelStatement::default()),
    Box::new(r#let::Let::default()),
    Box::new(r#return::Return::default()),
    Box::new(switch::Switch::default()),
    Box::new(throw::Throw::default()),
    Box::new(try_catch::TryCatch::default()),
    Box::new(try_catch_optional_catch_binding::TryCatchOptionalCatchBinding::default()),
    Box::new(var::VarDeclaration::default()),
    Box::new(r#while::While::default()),
    Box::new(with::With::default()),
  ]
}
