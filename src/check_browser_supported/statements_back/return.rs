use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_return_statement.push(walk_return_statement);
  },
  compat {
    name: "return",
    description: "return 语句",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_return_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ReturnStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "return",
    setup,
    should_ok_when_return_statement,
    r#"
  function getRectArea(width, height) {
    if (width > 0 && height > 0) {
      return width * height;
    }
    return 0;
  }
    "#,
    2,
  }
}
