#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Support<'a> {
  pub chrome: &'a str,
  pub edge: &'a str,
  pub firefox: &'a str,
  pub ie: &'a str,
  pub node: &'a str,
  pub safari: &'a str,
}
