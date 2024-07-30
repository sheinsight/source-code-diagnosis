#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Compat<'a> {
  pub name: &'a str,
  pub description: &'a str,
  pub mdn_url: &'a str,
  pub spec_url: &'a str,
  pub tags: &'a [&'a str],
  pub support: Support<'a>,
  pub status: Status,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Status {
  pub experimental: bool,
  pub standard_track: bool,
  pub deprecated: bool,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Support<'a> {
  pub chrome: &'a str,
  pub chrome_android: &'a str,
  pub firefox: &'a str,
  pub firefox_android: &'a str,
  pub safari: &'a str,
  pub safari_ios: &'a str,
  pub opera: &'a str,
  pub opera_android: &'a str,
  pub ie: &'a str,
  pub edge: &'a str,
  pub deno: &'a str,
  pub node: &'a str,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CompatBox {
  pub start: u32,
  pub end: u32,
  pub compat: Compat<'static>,
}
