use crate::create_compat_2;

create_compat_2! {
  ClassesExtends,
  compat {
    name: "classes.extends",
    description: "constructor function",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/extends",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "45",
      firefox_android: "45",
      safari: "9",
      safari_ios: "9",
      edge: "13",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::Class(class) = node.kind() {
        if class.super_class.is_some() {
            return true;
        }
    }
    false
  }
}

#[cfg(test)]
mod tests {

  use super::ClassesExtends;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_constructor:{
      setup: ClassesExtends::default(),
      source_code: r#"
        class A extends B { }
      "#,
      eq: [
        r#"class A extends B { }"#
      ],
      ne: []
    }
  }
}
