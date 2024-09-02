use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_sequence_expression.push(walk_sequence_expression);
  },
  compat {
    name: "operators_comma",
    description: "逗号运算符",
    tags: ["web-features:snapshot:ecmascript-1"],
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
  walk_sequence_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::SequenceExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_comma",
    setup,
    should_ok_when_use_comma,
    r#"
      let x = 1;
      x = (x++, x);
    "#,
    1
  }
}
