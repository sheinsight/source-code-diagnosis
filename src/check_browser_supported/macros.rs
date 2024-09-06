#[macro_export]
macro_rules! assert_ok_count {
  ($name:expr, $setup:expr, $($(#[$attr:meta])* $test_name:ident, $source_code:expr, $expected_count:expr),* $(,)?) => {
    $(
      $(#[$attr])*
      #[test]
      fn $test_name() {
        let source_code = $source_code;
        let mut v = crate::check_browser_supported::visitor::SyntaxVisitor::new(source_code, "index.ts");
        $setup(&mut v);
        let allocator = oxc_allocator::Allocator::default();
        let source_type = oxc_span::SourceType::default();
        let parser = oxc_parser::Parser::new(&allocator, source_code, source_type);
        let parse_return = parser.parse();
        oxc_ast::Visit::visit_program(&mut v, &parse_return.program);
        let count = v
            .context
            .usage
            .iter()
            .filter(|item| item.name == $name)
            .count();
        assert_eq!(count, $expected_count);
      }
    )*
  };
}

#[macro_export]
macro_rules! create_compat {
    (
        $setup_fn:ident,
        |$v:ident: &mut SyntaxVisitor| $setup_body:block,
        compat {
            name: $name:expr,
            description: $description:expr,
            tags: [$($tag:expr),*],
            support: {
                chrome: $chrome:expr,
                chrome_android: $chrome_android:expr,
                firefox: $firefox:expr,
                firefox_android: $firefox_android:expr,
                safari: $safari:expr,
                safari_ios: $safari_ios:expr,
                edge: $edge:expr,
                node: $node:expr,
                deno: $deno:expr,
            }
        },
        $(
            $walk_fn:ident,
            |$ctx:ident: &mut Context, $it:ident: &$expr_type:path $(, $extra_args:ident: $extra_types:ty)*| $body:block
        ),*
    ) => {
        use std::sync::OnceLock;
        use crate::check_browser_supported::{
            common::Context,
            compat::{Compat, CompatBox, Support},
            visitor::SyntaxVisitor,
        };

        static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

        fn get_compat() -> Compat {
            CONSTRUCTOR_COMPAT
                .get_or_init(|| {
                    Compat {
                        name: $name.to_string(),
                        description: $description.to_string(),
                        tags: vec![$($tag.to_string()),*],
                        support: Support {
                            chrome: $chrome.to_string(),
                            chrome_android: $chrome_android.to_string(),
                            firefox: $firefox.to_string(),
                            firefox_android: $firefox_android.to_string(),
                            safari: $safari.to_string(),
                            safari_ios: $safari_ios.to_string(),
                            edge: $edge.to_string(),
                            node: $node.to_string(),
                            deno: $deno.to_string(),
                        },
                    }
                })
                .clone()
        }

        $(
            fn $walk_fn($ctx: &mut Context, $it: &$expr_type $(, $extra_args: $extra_types)*) {
                if $body {
                    $ctx
                        .usage
                        .push(CompatBox::new($it.span.clone(), get_compat(), $ctx.file_path.clone() ));
                }
            }
        )*

        pub fn $setup_fn($v: &mut SyntaxVisitor) {
            $setup_body
        }
    };
}

#[macro_export]
macro_rules! create_compat_2 {
    (
        $struct_name:ident,
        compat {
            name: $name:expr,
            description: $description:expr,
            mdn_url: $mdn_url:expr,
            tags: [$($tag:expr),* $(,)?],
            support: {
                chrome: $chrome:expr,
                chrome_android: $chrome_android:expr,
                firefox: $firefox:expr,
                firefox_android: $firefox_android:expr,
                safari: $safari:expr,
                safari_ios: $safari_ios:expr,
                edge: $edge:expr,
                node: $node:expr,
                deno: $deno:expr,
            }
        },
        fn handle<'a>(&self, $source_code:ident: &str,$ast_node:ident: &AstNode<'a>, $nodes:ident: &AstNodes<'a>) -> bool $body:block
    ) => {
        use oxc_ast::AstKind;
        use oxc_semantic::{AstNode, AstNodes};
        use crate::check_browser_supported::compat::{Compat, CompatHandler, Support};

        pub struct $struct_name {
            pub compat: Compat,
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    compat: Compat {
                        name: $name.to_string(),
                        description: $description.to_string(),
                        mdn_url: $mdn_url.to_string(),
                        tags: vec![$($tag.to_string()),*],
                        support: Support {
                            chrome: $chrome.to_string(),
                            chrome_android: $chrome_android.to_string(),
                            firefox: $firefox.to_string(),
                            firefox_android: $firefox_android.to_string(),
                            safari: $safari.to_string(),
                            safari_ios: $safari_ios.to_string(),
                            edge: $edge.to_string(),
                            node: $node.to_string(),
                            deno: $deno.to_string(),
                        }
                    }
                }
            }
        }

        impl CompatHandler for $struct_name {
            fn handle<'a>(&self, $source_code: &str,$ast_node: &AstNode<'a>, $nodes: &AstNodes<'a>) -> bool $body

            fn get_compat(&self) -> &Compat {
                &self.compat
            }
        }
    };
}

#[macro_export]
macro_rules! assert_source_seg {
    (
        $(
            $test_name:ident: {
                setup: $compat_handler:expr,
                source_code: $source_code:expr,
                eq: [$($ok_expected:expr),* $(,)?],
                ne: [$($fail_expected:expr),* $(,)?]
            }
        ),* $(,)?
    ) => {
        $(
            #[test]
            fn $test_name() {
                let source_code = $source_code;

                let allocator = oxc_allocator::Allocator::default();
                let source_type = oxc_span::SourceType::default();

                let ret = oxc_parser::Parser::new(&allocator, source_code, source_type).parse();
                let program = allocator.alloc(ret.program);

                let semantic = oxc_semantic::SemanticBuilder::new(&source_code, source_type)
                    .build(program)
                    .semantic;

                let nodes = semantic.nodes();

                let mut result: Vec<crate::check_browser_supported::compat::CompatBox> = Vec::new();
                let compat_handler = $compat_handler;
                let mut source_seg: Vec<String> = Vec::new();

                for node in nodes.iter() {
                    if crate::check_browser_supported::compat::CompatHandler::handle(
                      &compat_handler,
                      source_code,
                      node,
                      nodes,
                    ) {
                        let span = oxc_span::GetSpan::span(&node.kind());
                        let start_position =
                        crate::utils::offset_to_position(span.start as usize, &source_code).unwrap();
                        let end_position =
                          crate::utils::offset_to_position(span.end as usize, &source_code).unwrap();
                        result.push(
                            crate::check_browser_supported::compat::CompatBox::new(
                                span,
                                crate::utils::Location {
                                    start: crate::utils::Position {
                                      line: start_position.line,
                                      col: start_position.character,
                                    },
                                    end: crate::utils::Position {
                                      line: end_position.line,
                                      col: end_position.character,
                                    },
                                },
                                crate::check_browser_supported::compat::CompatHandler::get_compat(&compat_handler).clone(),
                                "".to_string(),
                            )
                        );
                        source_seg.push(
                            crate::check_browser_supported::compat::AstNodeHelper::text(
                            node,
                            &source_code,
                            ),
                      )
                    }
                  }

                let ok_expect_source_seg:Vec<&str> = vec![$($ok_expected),*];
                let fail_expect_source_seg:Vec<&str> = vec![$($fail_expected),*];

                assert_eq!(source_seg.len(), ok_expect_source_seg.len());

                for (index, seg) in source_seg.iter().enumerate() {
                    assert_eq!(ok_expect_source_seg.get(index).unwrap().trim(), seg.trim());
                }

                for fail_seg in fail_expect_source_seg.iter() {
                    for seg in source_seg.iter() {
                        assert_ne!(fail_seg.trim(), seg.trim());
                    }
                }

                assert_eq!(result.len(), ok_expect_source_seg.len());
            }
        )*
    };
}
