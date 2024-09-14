use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  OperatorsConditional,
  compat {
    name: "operators.conditional",
    description: "条件（三元）运算符",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Conditional_Operator",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "4",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::ConditionalExpression(_))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsConditional;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_conditional_operator:{
      setup: OperatorsConditional::default(),
      source_code: r#"
        const age = 26;
        const beverage = age >= 21 ? "Beer" : "Juice";
        console.log(beverage);
      "#,
      eq: [
        r#"age >= 21 ? "Beer" : "Juice""#,
      ],
      ne: []
    }
  }
}
