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
  constructor::setup_class_constructor(v);
  extends::setup_class_extends(v);
  private_class_fields_in::setup_private_fields_in(v);
  private_class_fields::setup_private_fields(v);
  private_class_methods::setup_private_method(v);
  public_class_fields::setup_public_fields(v);
  static_class_fields::setup_static_fields(v);
  static_initialization_blocks::setup_static_initialization_block(v);
  r#static::setup_static(v);
}
