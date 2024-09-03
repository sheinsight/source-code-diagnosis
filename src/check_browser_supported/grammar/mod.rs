use super::compat::CompatHandler;

pub mod array_literals;
pub mod binary_numeric_literals;
pub mod boolean_literals;
pub mod decimal_numeric_literals;
pub mod hashbang_comments;

pub fn setup() -> Vec<Box<dyn CompatHandler>> {
  vec![
    Box::new(array_literals::ArrayLiterals::default()),
    Box::new(binary_numeric_literals::BinaryNumericLiterals::default()),
    Box::new(boolean_literals::BooleanLiterals::default()),
    Box::new(decimal_numeric_literals::DecimalNumericLiterals::default()),
    Box::new(hashbang_comments::HashbangComments::default()),
  ]
}
