use constructor::ClassesConstructor;
use extends::ClassesExtends;
use private_class_fields::ClassesPrivateClassFields;
use private_class_fields_in::ClassesPrivateClassFieldsIn;
use private_class_methods::ClassesPrivateClassMethods;
use public_class_fields::ClassesPublicClassFields;
use r#static::ClassesStatic;
use static_class_fields::ClassesStaticClassFields;

use super::compat::CompatHandler;

pub mod classes;
pub mod constructor;
pub mod extends;
pub mod private_class_fields;
pub mod private_class_fields_in;
pub mod private_class_methods;
pub mod public_class_fields;
pub mod r#static;
pub mod static_class_fields;
pub mod static_initialization_blocks;

pub fn setup() -> Vec<Box<dyn CompatHandler>> {
  vec![
    Box::new(ClassesConstructor::default()),
    Box::new(ClassesExtends::default()),
    Box::new(ClassesPrivateClassFields::default()),
    Box::new(ClassesPrivateClassFieldsIn::default()),
    Box::new(ClassesPrivateClassMethods::default()),
    Box::new(ClassesPublicClassFields::default()),
    Box::new(ClassesStaticClassFields::default()),
    Box::new(ClassesStatic::default()),
  ]
}
