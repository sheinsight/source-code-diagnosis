use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_chain_expression.push(walk_chain_expression);
  },
  compat {
    name: "operators_optional_chaining",
    description: "Optional chaining operator (<code>?.</code>)",
    tags: ["web-features:snapshot:ecmascript-2020"],
    support: {
      chrome: "80",
      chrome_android: "80",
      firefox: "74",
      firefox_android: "74",
      safari: "13.1",
      safari_ios: "13.1",
      edge: "80",
      node: "14.0.0",
      deno: "1.0",
    }
  },
  walk_chain_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ChainExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_optional_chaining",
    setup,
    should_ok_when_chain_expression_optional_chaining,
    r#"
    const user = {
      name: '1',
      address: {
        city: '2'
      }
    };
    console.log(user?.address?.city);
    console.log(user?.contact?.email);
    "#,
    2
  }
}
