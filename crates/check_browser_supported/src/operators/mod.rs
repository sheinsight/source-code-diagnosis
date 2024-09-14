use super::compat::CompatHandler;

pub mod addition;
pub mod addition_assignment;
pub mod assignment;
pub mod async_function;
pub mod async_generator_function;
pub mod r#await;
pub mod bitwise_and;
pub mod bitwise_and_assignment;
pub mod bitwise_not;
pub mod bitwise_or;
pub mod bitwise_or_assignment;
pub mod bitwise_xor;
pub mod bitwise_xor_assignment;
pub mod class;
pub mod comma;
pub mod conditional;
pub mod decrement;
pub mod delete;
pub mod destructuring;
pub mod destructuring_computed_property_names;
pub mod destructuring_rest_in_arrays;
pub mod destructuring_rest_in_objects;
pub mod division;
pub mod division_assignment;
pub mod equality;
pub mod exponentiation;
pub mod exponentiation_assignment;
pub mod function;
pub mod function_trailing_comma;
pub mod generator_function;
pub mod generator_function_trailing_comma;
pub mod greater_than;
pub mod greater_than_or_equal;
pub mod grouping;
pub mod import;
pub mod import_meta;
pub mod import_meta_resolve;
pub mod import_options_parameter;
pub mod r#in;
pub mod increment;
pub mod inequality;
pub mod instanceof;
pub mod left_shift;
pub mod left_shift_assignment;
pub mod less_than;
pub mod less_than_or_equal;
pub mod logical_and;
pub mod logical_and_assignment;
pub mod logical_not;
pub mod logical_or;
pub mod logical_or_assignment;
pub mod multiplication;
pub mod multiplication_assignment;
pub mod new;
pub mod new_target;
pub mod null;
pub mod nullish_coalescing;
pub mod nullish_coalescing_assignment;
pub mod object_initializer;
pub mod object_initializer_computed_property_names;
pub mod object_initializer_shorthand_method_names;
pub mod object_initializer_shorthand_property_names;
pub mod object_initializer_spread_properties;
pub mod optional_chaining;
pub mod property_accessors;
pub mod r_super;
pub mod remainder;
pub mod remainder_assignment;
pub mod right_shift;
pub mod right_shift_assignment;
pub mod spread;
pub mod spread_spread_in_arrays;
pub mod spread_spread_in_function_calls;
pub mod spread_spread_in_object_literals;
pub mod strict_equality;
pub mod strict_inequality;
pub mod subtraction;
pub mod subtraction_assignment;
pub mod this;
pub mod r#typeof;
pub mod unary_negation;
pub mod unary_plus;
pub mod unsigned_right_shift;
pub mod unsigned_right_shift_assignment;
pub mod void;
pub mod r#yield;
pub mod yield_star;

pub fn setup() -> Vec<Box<dyn CompatHandler>> {
  vec![
    Box::new(addition_assignment::AdditionAssignment::default()),
    Box::new(addition::Addition::default()),
    Box::new(assignment::Assignment::default()),
    Box::new(async_function::AsyncFunction::default()),
    Box::new(async_generator_function::AsyncGeneratorFunction::default()),
    Box::new(r#await::OperatorsAwait::default()),
    Box::new(bitwise_and_assignment::BitwiseAndAssignment::default()),
    Box::new(bitwise_and::BitwiseAnd::default()),
    Box::new(bitwise_not::BitwiseNot::default()),
    Box::new(bitwise_or_assignment::BitwiseOrAssignment::default()),
    Box::new(bitwise_or::BitwiseOr::default()),
    Box::new(bitwise_xor_assignment::BitwiseXorAssignment::default()),
    Box::new(bitwise_xor::BitwiseXor::default()),
    Box::new(class::ClassesExpression::default()),
    Box::new(comma::OperatorsComma::default()),
    Box::new(conditional::OperatorsConditional::default()),
    Box::new(decrement::Decrement::default()),
    Box::new(delete::Delete::default()),
    Box::new(
      destructuring_computed_property_names::DestructuringComputedPropertyNames::default(),
    ),
    Box::new(
      destructuring_rest_in_arrays::DestructuringRestInArrays::default(),
    ),
    Box::new(
      destructuring_rest_in_objects::DestructuringRestInObjects::default(),
    ),
    Box::new(destructuring::Destructuring::default()),
    Box::new(division_assignment::DivisionAssignment::default()),
    Box::new(division::Division::default()),
    Box::new(equality::OperatorsEquality::default()),
    Box::new(exponentiation::Exponentiation::default()),
    Box::new(exponentiation_assignment::ExponentiationAssignment::default()),
    Box::new(function_trailing_comma::FunctionTrailingComma::default()),
    Box::new(function::OperatorsFunctionExpression::default()),
    Box::new(generator_function_trailing_comma::GeneratorFunctionTrailingComma::default()),
    Box::new(generator_function::OperatorsGeneratorFunction::default()),
    Box::new(greater_than_or_equal::OperatorsGreaterThanOrEqual::default()),
    Box::new(greater_than::OperatorsGreaterThan::default()),
    Box::new(grouping::OperatorsGrouping::default()),
    Box::new(import_meta_resolve::OperatorsImportMetaResolve::default()),
    Box::new(import_meta::OperatorsImportMeta::default()),
    Box::new(import_options_parameter::ImportOptionsParameter::default()),
    Box::new(import::OperatorsImport::default()),
    Box::new(r#in::OperatorsIn::default()),
    Box::new(increment::OperatorsIncrement::default()),
    Box::new(inequality::OperatorsInequality::default()),
    Box::new(instanceof::OperatorsInstanceof::default()),
    Box::new(left_shift_assignment::LeftShiftAssignment::default()),
    Box::new(left_shift::OperatorsLeftShift::default()),
    Box::new(less_than_or_equal::OperatorsLessThanOrEqual::default()),
    Box::new(less_than::OperatorsLessThan::default()),
    Box::new(logical_and_assignment::LogicalAndAssignment::default()),
    Box::new(logical_and::OperatorsLogicalAnd::default()),
    Box::new(logical_not::OperatorsLogicalNot::default()),
    Box::new(logical_or_assignment::OperatorsLogicalOrAssignment::default()),
    Box::new(logical_or::OperatorsLogicalOr::default()),
    Box::new(multiplication_assignment::MultiplicationAssignment::default()),
    Box::new(multiplication::Multiplication::default()),
    Box::new(new_target::OperatorsNewTarget::default()),
    Box::new(new::OperatorsNew::default()),
    Box::new(null::NullLiteral::default()),
    Box::new(nullish_coalescing_assignment::NullishCoalescingAssignment::default()),
    Box::new(nullish_coalescing::OperatorsNullishCoalescing::default()),
    Box::new(object_initializer_computed_property_names::ObjectInitializerComputedPropertyNames::default()),
    Box::new(object_initializer_shorthand_method_names::ObjectInitializerShorthandMethodNames::default()),
    Box::new(object_initializer_shorthand_property_names::ObjectInitializerShorthandPropertyNames::default()),
    Box::new(object_initializer_spread_properties::ObjectInitializerSpreadProperties::default()),
    Box::new(object_initializer::ObjectInitializer::default()),
    Box::new(optional_chaining::OperatorsOptionalChaining::default()),
    Box::new(property_accessors::OperatorsPropertyAccessors::default()),
    Box::new(r_super::OperatorsSuper::default()),
    Box::new(remainder_assignment::RemainderAssignment::default()),
    Box::new(remainder::OperatorsRemainder::default()),
    Box::new(right_shift_assignment::RightShiftAssignment::default()),
    Box::new(right_shift::OperatorsRightShift::default()),
    Box::new(spread_spread_in_arrays::SpreadInArrays::default()),
    Box::new(spread_spread_in_function_calls::SpreadInFunctionCalls::default()),
    Box::new(spread_spread_in_object_literals::SpreadInObjectLiterals::default()),
    Box::new(spread::Spread::default()),
    Box::new(strict_equality::OperatorsStrictEquality::default()),
    Box::new(strict_inequality::OperatorsStrictInequality::default()),
    Box::new(subtraction_assignment::SubtractionAssignment::default()),
    Box::new(subtraction::OperatorsSubtraction::default()),
    Box::new(this::OperatorsThis::default()),
    Box::new(r#typeof::OperatorsTypeof::default()),
    Box::new(unary_negation::OperatorsUnaryNegation::default()),
    Box::new(unary_plus::OperatorsUnaryPlus::default()),
    Box::new(unsigned_right_shift_assignment::UnsignedRightShiftAssignment::default()),
    Box::new(unsigned_right_shift::OperatorsUnsignedRightShift::default()),
    Box::new(void::OperatorsVoid::default()),
    Box::new(yield_star::OperatorsYieldStar::default()),
    Box::new(r#yield::OperatorsYield::default()),
  ]
}
