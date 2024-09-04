use crate::create_compat_2;

create_compat_2! {
  GeneratorFunctionNotConstructableWithNew,
  compat {
    name: "statements.generator_function.not_constructable_with_new",
    description: "生成器函数不能用 new 关键字构造（ES2016）",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/function*#Cannot_use_new_with_a_generator_function",
    tags: ["web-features:snapshot:ecmascript-2016"],
    support: {
      chrome: "50.0.0",
      chrome_android: "50.0.0",
      firefox: "43.0.0",
      firefox_android: "43.0.0",
      safari: "10.0.0",
      safari_ios: "10.0.0",
      edge: "13.0.0",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::Function(function) if function.generator)
  }
}

#[cfg(test)]
mod tests {
  use super::GeneratorFunctionNotConstructableWithNew;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_generator_function_used_with_new:{
      setup: GeneratorFunctionNotConstructableWithNew::default(),
      source_code: r#"
        function* generator() {
          yield 1;
          yield 2;
        }
        const gen = new generator();
      "#,
      eq: [
        r#"function* generator() {
          yield 1;
          yield 2;
        }"#,
      ],
      ne: []
    },

    should_not_ok_when_generator_function_not_used_with_new:{
      setup: GeneratorFunctionNotConstructableWithNew::default(),
      source_code: r#"
        function* generator() {
          yield 1;
          yield 2;
        }
        const gen = generator();
      "#,
      eq: [
        r#"function* generator() {
          yield 1;
          yield 2;
        }"#,
      ],
      ne: [
        r#"generator()"#,
      ]
    },

    should_not_ok_when_regular_function_used_with_new:{
      setup: GeneratorFunctionNotConstructableWithNew::default(),
      source_code: r#"
        function regularFunction() {
          return 1;
        }
        const instance = new regularFunction();
      "#,
      eq: [],
      ne: [
        r#"regularFunction()"#,
      ]
    }
  }
}
