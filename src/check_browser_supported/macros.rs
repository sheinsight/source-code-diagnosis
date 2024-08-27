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
