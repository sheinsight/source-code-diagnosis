use arguments::Arguments;
use callee::Callee;
use functions::FunctionsDeclarations;
use length::Length;

use super::compat::CompatHandler;

pub mod arguments;
pub mod callee;
pub mod functions;
pub mod iterator;
pub mod length;

pub fn setup() -> Vec<Box<dyn CompatHandler>> {
  vec![
    Box::new(FunctionsDeclarations::default()),
    Box::new(Arguments::default()),
    Box::new(Callee::default()),
    Box::new(Length::default()),
    Box::new(iterator::Iterator::default()),
  ]
}
