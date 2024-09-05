use super::compat::CompatHandler;

pub mod array_literals;
pub mod binary_numeric_literals;
pub mod boolean_literals;
pub mod decimal_numeric_literals;
pub mod hashbang_comments;
pub mod hexadecimal_escape_sequences;
pub mod hexadecimal_numeric_literals;
pub mod null_literal;
pub mod numeric_separators;
pub mod octal_numeric_literals;
pub mod regular_expression_literals;
pub mod shorthand_object_literals;
pub mod string_literals;
pub mod template_literals;
pub mod template_literals_template_literal_revision;
pub mod trailing_commas;
pub mod trailing_commas_trailing_commas_in_dynamic_import;
pub mod trailing_commas_trailing_commas_in_functions;
pub mod trailing_commas_trailing_commas_in_object_literals;
pub mod unicode_escape_sequences;
pub mod unicode_point_escapes;

pub fn setup() -> Vec<Box<dyn CompatHandler>> {
  vec![
    Box::new(array_literals::ArrayLiterals::default()),
    Box::new(binary_numeric_literals::BinaryNumericLiterals::default()),
    Box::new(boolean_literals::BooleanLiterals::default()),
    Box::new(decimal_numeric_literals::DecimalNumericLiterals::default()),
    Box::new(hashbang_comments::HashbangComments::default()),
    Box::new(null_literal::NullLiteral::default()),
    Box::new(numeric_separators::NumericSeparators::default()),
    Box::new(octal_numeric_literals::OctalNumericLiterals::default()),
    Box::new(regular_expression_literals::RegularExpressionLiterals::default()),
    Box::new(shorthand_object_literals::ShorthandObjectLiterals::default()),
    Box::new(string_literals::StringLiterals::default()),
    Box::new(template_literals::TemplateLiterals::default()),
    Box::new(trailing_commas_trailing_commas_in_dynamic_import::TrailingCommasInDynamicImport::default()),
    Box::new(trailing_commas_trailing_commas_in_functions::TrailingCommasInFunctions::default()),
    Box::new(trailing_commas_trailing_commas_in_object_literals::TrailingCommasInObjectLiterals::default()),
    Box::new(trailing_commas::TrailingCommas::default()),
    // Box::new(template_literals_template_literal_revision::TemplateLiteralsTemplateLiteralRevision::default()),
    // Box::new(unicode_escape_sequences::UnicodeEscapeSequences::default()),
    // Box::new(unicode_point_escapes::UnicodePointEscapes::default()),
    // Box::new(hexadecimal_escape_sequences::HexadecimalEscapeSequences::default()),
    // Box::new(hexadecimal_numeric_literals::HexadecimalNumericLiterals::default()),
  ]
}
