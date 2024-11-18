#[macro_export]
macro_rules! create_compat {
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
        use oxc_semantic::{AstNode, AstNodes};
        use crate::compat::{Compat, CompatHandler, Support};

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

                let semantic = oxc_semantic::SemanticBuilder::new(&source_code)
                    .build(program)
                    .semantic;

                let nodes = semantic.nodes();

                let mut result: Vec<crate::compat::CompatBox> = Vec::new();
                let compat_handler = $compat_handler;
                let mut source_seg: Vec<String> = Vec::new();

                for node in nodes.iter() {
                    if crate::compat::CompatHandler::handle(
                      &compat_handler,
                      source_code,
                      node,
                      nodes,
                    ) {
                        let span = oxc_span::GetSpan::span(&node.kind());
                        // let start_position =
                        // crate::utils::offset_to_position(span.start as usize, &source_code).unwrap();
                        // let end_position =
                        //   crate::utils::offset_to_position(span.end as usize, &source_code).unwrap();
                        result.push(
                            crate::compat::CompatBox::new(
                                span,
                                beans::Location {
                                    start: beans::Position{
                                        line:0,
                                        col:0,
                                    },
                                    end: beans::Position{
                                        line:0,
                                        col:0,
                                    },
                                },
                                crate::compat::CompatHandler::get_compat(&compat_handler).clone(),
                                "".to_string(),
                            )
                        );
                        source_seg.push(crate::compat::AstNodeHelper::text(node,&source_code))
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
