use crate::create_compat_2;

create_compat_2! {
  HashbangComments,
  compat {
    name: "hashbang_comments",
    description: "Hashbang (<code>#!</code>) comment syntax",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Hashbang_comments",
    tags: [
      "web-features:snapshot:ecmascript-2023"
    ],
    support: {
      chrome: "74.0.0",
      chrome_android: "74.0.0",
      firefox: "67.0.0",
      firefox_android: "67.0.0",
      safari: "13.1.0",
      safari_ios: "13.1.0",
      edge: "74.0.0",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::Program(program) if program.hashbang.is_some())
  }
}

#[cfg(test)]
mod tests {
  use super::HashbangComments;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_hashbang_comments:{
      setup: HashbangComments::default(),
      source_code: r#"#!/usr/bin/env node
      console.log("Hello world");"#,
      eq: [
        r#"#!/usr/bin/env node
      console.log("Hello world");"#,
      ],
      ne: []
    },

    should_fail_when_no_hashbang_comments:{
      setup: HashbangComments::default(),
      source_code: r#"
        console.log("Hello world");
      "#,
      eq: [],
      ne: []
    }
  }
}
