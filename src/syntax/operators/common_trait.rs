use crate::syntax::compat::CompatBox;

pub trait CommonTrait {
  fn get_cache(&self) -> Vec<CompatBox>;
}
