use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_identifier_reference.push(arguments_iterator);
  },
  compat {
    name: "arguments_iterator",
    description: "arguments is iterable",
    tags: [
      "web-features:arguments-iterator",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "52",
      chrome_android: "52",
      firefox: "46",
      firefox_android: "46",
      safari: "9",
      safari_ios: "9",
      edge: "12",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  arguments_iterator,
  |ctx: &mut Context, it: &oxc_ast::ast::IdentifierReference| {
    it.name == "arguments" && ctx.stack.last().map_or(false, |p| matches!(p, AstKind::ForOfStatement(_)))
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "arguments_iterator",
    setup,

    should_ok_when_use_arguments_iterator,
    r#"
      function f() {
        for (const letter of arguments) {
          console.log(letter);
        }
      }
      f("w", "y", "k", "o", "p");
    "#,
    1,

    should_ok_when_not_use_arguments_iterator,
    r#"
      function f() {
        for (const letter of []) {
          console.log(letter);
        }
      }
      f("w", "y", "k", "o", "p");
    "#,
    0,

    should_ok_when_use_arguments_iterator_in_arrow_function,
    r#"
      const f = () => {
        for (const letter of arguments) {
          console.log(letter);
        }
      }
      f("w", "y", "k", "o", "p");
    "#,
    1,

    should_ok_when_use_arguments_iterator_in_arrow_function_with_arguments,
    r#"
      const f = (arguments) => {
        for (const letter of arguments) {
          console.log(letter);
        }
      }
      f("w", "y", "k", "o", "p");
    "#,
    1,
  }
}
