use crate::create_compat_2;

create_compat_2! {
  TryCatchOptionalCatchBinding,
  compat {
    name: "statements.try_catch.optional_catch_binding",
    description: "可选的 catch 绑定",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/try...catch#Optional_catch_binding",
    tags: ["web-features:snapshot:ecmascript-2019"],
    support: {
      chrome: "66",
      chrome_android: "66",
      firefox: "58",
      firefox_android: "58",
      safari: "11.1",
      safari_ios: "11.1",
      edge: "66",
      node: "10.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::CatchClause(catch_clause) = node.kind() {
      catch_clause.param.is_none()
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::TryCatchOptionalCatchBinding;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_optional_catch_binding:{
      setup: TryCatchOptionalCatchBinding::default(),
      source_code: r#"
        try {
          // 一些可能抛出错误的代码
        } catch {
          console.error("发生了错误");
        }
      "#,
      eq: [
        r#"catch {
          console.error("发生了错误");
        }"#,
      ],
      ne: []
    },

    should_fail_when_no_use_optional_catch_binding:{
      setup: TryCatchOptionalCatchBinding::default(),
      source_code: r#"
        try {
          // 一些可能抛出错误的代码
        } catch(error) {
          console.error("发生了错误:", error);
        }
      "#,
      eq: [],
      ne: [
        r#"catch(error) {
          console.error("发生了错误:", error);
        }"#,
      ]
    }
  }
}
