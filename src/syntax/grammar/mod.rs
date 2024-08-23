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
  array_literals::setup(v);
  binary_numeric_literals::setup(v);
  boolean_literals::setup(v);
  decimal_numeric_literals::setup(v);
  hashbang_comments::setup(v);
  // hexadecimal_escape_sequences::setup_hexadecimal_escape_sequences(v);
  hexadecimal_numeric_literals::setup(v);
  null_literal::setup(v);
  numeric_separators::setup(v);
  octal_numeric_literals::setup(v);
  regular_expression_literals::setup(v);
  shorthand_object_literals::setup(v);
  string_literals::setup(v);
  template_literals_template_literal_revision::setup(v);
  template_literals::setup(v);
  trailing_commas_trailing_commas_in_dynamic_import::setup(v);
  trailing_commas_trailing_commas_in_functions::setup(v);
  trailing_commas_trailing_commas_in_object_literals::setup(v);
  trailing_commas::setup(v);
  // unicode_escape_sequences::setup_unicode_escape_sequences(v);
  // unicode_point_escapes::setup_unicode_point_escapes(v);
}
