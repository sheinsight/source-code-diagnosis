use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_reg_exp_literal.push(walk_reg_exp_literal);
  },
  compat {
    name: "regular_expression_literals",
    description: "正则表达式字面量 (/ab+c/g)",
    tags: [
      "web-features:snapshot:ecmascript-3"
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
  walk_reg_exp_literal,
  |ctx: &mut Context, it: &oxc_ast::ast::RegExpLiteral| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "regular_expression_literals",
    setup,

    should_ok_when_use_regular_expression_literals,
    r#"
      /ab+c/g;
      /[/]/;
    "#,
    2,
  }
}
