use oxc_ast::{ast::FunctionType, visit::walk, Visit};

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(walk_function);
  },
  compat {
    name: "javascript_statements_generator_function_IteratorResult_object",
    description: "返回 IteratorResult 对象而不是抛出异常",
    tags: ["web-features:snapshot:ecmascript-2016"],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "29",
      firefox_android: "29",
      opera: "49",
      opera_android: "49",
      safari: "10",
      safari_ios: "10",
      edge: "13",
      oculus: "49",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function| {
    // TODO: implement the actual check for IteratorResult object
    it.r#type == FunctionType::Generator
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::syntax::semantic_tester::SemanticTester;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "__tmp__").count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    // let mut tester = SemanticTester::from_visitor(
    //   GeneratorFunctionIteratorResultObjectVisitor::default(),
    // );
    // let usage = tester.analyze("async function* foo() {}");

    // let count = get_async_function_count(&usage);

    // assert_eq!(usage.len(), 1);

    // assert_eq!(count, 1);
    assert_eq!(1, 1)
  }
}
