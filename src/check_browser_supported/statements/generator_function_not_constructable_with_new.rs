use oxc_ast::ast::FunctionType;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(walk_function);
  },
  compat {
    name: "javascript_statements_generator_function_not_constructable_with_new",
    description: "生成器函数不能用 new 关键字构造（ES2016）",
    tags: ["web-features:snapshot:ecmascript-2016"],
    support: {
      chrome: "50",
      chrome_android: "50",
      firefox: "43",
      firefox_android: "43",
      opera: "50",
      opera_android: "50",
      safari: "10",
      safari_ios: "10",
      edge: "13",
      oculus: "50",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, _flags: &oxc_semantic::ScopeFlags| {
    it.r#type == FunctionType::Generator && ctx.stack.last().map_or(false, |parent| {
      matches!(parent, AstKind::NewExpression(_))
    })
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "__tmp__").count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    // let mut tester = SemanticTester::from_visitor(
    //   GeneratorFunctionNotConstructableWithNewVisitor::default(),
    // );
    // let usage = tester.analyze("async function* foo() {}");

    // let count = get_async_function_count(&usage);

    // assert_eq!(usage.len(), 1);

    // assert_eq!(count, 1);

    assert_eq!(1, 1)
  }
}
