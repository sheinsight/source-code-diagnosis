use oxc_ast::AstKind;

use crate::create_compat_2;

create_compat_2! {
  Spread,
  compat {
    name: "operators.spread",
    description: "Spread syntax (...)",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Spread_syntax",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "46",
      chrome_android: "46",
      firefox: "16",
      firefox_android: "16",
      safari: "8",
      safari_ios: "8",
      edge: "12",
      node: "5.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::SpreadElement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::Spread;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_spread_in_callee_function:{
      setup: Spread::default(),
      source_code: r#"
        console.log(sum(...numbers));
      "#,
      eq: [
        r#"...numbers"#,
      ],
      ne: []
    },

    should_ok_when_spread_in_object_pattern:{
      setup: Spread::default(),
      source_code: r#"
        const obj = { ...true, ...10 };
      "#,
      eq: [
        r#"...true"#,
        r#"...10"#,
      ],
      ne: []
    },

    should_ok_when_spread_in_array_literal:{
      setup: Spread::default(),
      source_code: r#"
        const arr = [...[1, 2, 3], 4, 5];
      "#,
      eq: [
        r#"...[1, 2, 3]"#,
      ],
      ne: []
    }
  }
}
