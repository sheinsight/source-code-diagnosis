use oxc_ast::AstKind;

use crate::create_compat_2;

create_compat_2! {
  OperatorsPropertyAccessors,
  compat {
    name: "operators.property_accessors",
    description: "属性访问器",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Property_Accessors",
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
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::MemberExpression(_))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsPropertyAccessors;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_computed_member_expression:{
      setup: OperatorsPropertyAccessors::default(),
      source_code: r#"
        const car = { brand: 'Toyota' };
        console.log(car['brand']);
      "#,
      eq: [
        r#"car['brand']"#,
        r#"console.log"#,
      ],
      ne: []
    },

    should_ok_when_static_member_expression:{
      setup: OperatorsPropertyAccessors::default(),
      source_code: r#"
        const car = { brand: 'Toyota' };
        console.log(car.brand);
      "#,
      eq: [
        r#"car.brand"#,
        r#"console.log"#,
      ],
      ne: []
    }
  }
}
