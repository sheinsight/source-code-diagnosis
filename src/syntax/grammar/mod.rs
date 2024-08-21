use super::visitor::SyntaxVisitor;

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

pub fn setup_grammar(v: &mut SyntaxVisitor) {
  array_literals::setup_array_literals(v);
  binary_numeric_literals::setup_binary_numeric_literals(v);
  boolean_literals::setup_boolean_literals(v);
  decimal_numeric_literals::setup_decimal_numeric_literals(v);
  hashbang_comments::setup_hashbang_comments(v);
  hexadecimal_escape_sequences::setup_hexadecimal_escape_sequences(v);
  hexadecimal_numeric_literals::setup_hexadecimal_numeric_literals(v);
  null_literal::setup_null_literal(v);
  numeric_separators::setup_numeric_separators(v);
  octal_numeric_literals::setup_octal_numeric_literals(v);
  regular_expression_literals::setup_regular_expression_literals(v);
  shorthand_object_literals::setup_shorthand_object_literals(v);
  string_literals::setup_string_literals(v);
  template_literals_template_literal_revision::setup_template_literals_template_literal_revision(v);
  template_literals::setup_template_literals(v);
  trailing_commas_trailing_commas_in_dynamic_import::setup_trailing_commas_trailing_commas_in_dynamic_import(v);
  trailing_commas_trailing_commas_in_functions::setup_trailing_commas_trailing_commas_in_functions(v);
  trailing_commas_trailing_commas_in_object_literals::setup_trailing_commas_trailing_commas_in_object_literals(v);
  trailing_commas::setup_trailing_commas(v);
  unicode_escape_sequences::setup_unicode_escape_sequences(v);
  unicode_point_escapes::setup_unicode_point_escapes(v);
}
