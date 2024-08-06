use super::compat::CompatBox;

pub trait CommonTrait {
  fn get_usage(&self) -> Vec<CompatBox>;
}
