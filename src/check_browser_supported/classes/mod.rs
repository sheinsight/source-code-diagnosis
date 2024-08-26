use super::visitor::SyntaxVisitor;

pub mod constructor;
pub mod extends;
pub mod private_class_fields;
pub mod private_class_fields_in;
pub mod private_class_methods;
pub mod public_class_fields;
pub mod r#static;
pub mod static_class_fields;
pub mod static_initialization_blocks;

pub fn setup_classes(v: &mut SyntaxVisitor) {
  constructor::setup(v);
  extends::setup(v);
  private_class_fields_in::setup(v);
  private_class_fields::setup(v);
  private_class_methods::setup(v);
  public_class_fields::setup(v);
  static_class_fields::setup(v);
  static_initialization_blocks::setup(v);
  r#static::setup(v);
}
