use oxc_allocator::Allocator;
use oxc_semantic::{SemanticBuilder, SemanticBuilderReturn};
use oxc_span::SourceType;

pub struct SemanticChecker<'a> {
  /// Memory arena that AST and [`Semantic`] will store data in.
  allocator: Allocator,
  /// Source type of the test case.
  source_type: SourceType,
  /// The source text of the test case.
  source_text: &'a str,
  /// Build a [`ControlFlowGraph`]?
  ///
  /// [`ControlFlowGraph`]: oxc_cfg::ControlFlowGraph
  cfg: bool,
  /// Expect semantic analysis to produce errors.
  ///
  /// Default is `false`.
  expect_errors: bool,
}

impl<'a> SemanticChecker<'a> {
  pub fn new(source_text: &'a str, source_type: SourceType) -> Self {
    Self {
      allocator: Allocator::default(),
      source_type,
      source_text,
      cfg: false,
      expect_errors: false,
    }
  }

  pub fn js(source_text: &'static str) -> Self {
    Self::new(source_text, SourceType::default().with_module(true))
  }

  pub fn build(&self) -> SemanticBuilderReturn<'_> {
    let parse = oxc_parser::Parser::new(
      &self.allocator,
      self.source_text,
      self.source_type,
    )
    .parse();

    let program = self.allocator.alloc(parse.program);
    SemanticBuilder::new(self.source_text, self.source_type)
      .with_check_syntax_error(true)
      .with_trivias(parse.trivias)
      .with_cfg(self.cfg)
      .build(program)
  }

  pub fn has_class(&self) {}
}

#[cfg(test)]
mod tests {

  use oxc_allocator::Allocator;
  use oxc_ast::Visit;
  use oxc_parser::Parser;
  use oxc_semantic::SemanticBuilder;
  use oxc_span::SourceType;

  use super::SemanticChecker;

  #[test]
  fn should_ok() {
    // let checker = SemanticChecker::js(
    //   "
    //   console.log(1);
    //   function hello(){
    //   class Foo {
    //     #privateProperty = 1;
    //     publicProperty = 2;

    //     constructor() {} // this method is skip
    //     a() {}
    //     set b(v) {}
    //     get b() {}
    //   }
    //   }

    // ",
    // );

    // checker.has_class();

    // let x = checker.build();

    // x.semantic.nodes().iter().for_each(|item| {
    //   // item.

    //   // println!("{:?}", item.scope_id());
    //   // SemanticBuilder::new(source_text, source_type)
    // });

    // println!("{:?}", x.semantic.nodes());
  }
}
