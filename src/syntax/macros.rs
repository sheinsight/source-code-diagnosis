#[macro_export]
macro_rules! create_compat {
    (
        $json_path:expr,
        $setup_fn:ident,
        |$v:ident: &mut SyntaxVisitor| $setup_body:block,
        $(
            $walk_fn:ident,
            |$ctx:ident: &mut Context, $it:ident: &$expr_type:path $(, $extra_args:ident: $extra_types:ty)*| $body:block
        ),*
    ) => {
        use std::sync::OnceLock;
        use serde_json5::from_str;
        use crate::syntax::{
            common::Context,
            compat::{Compat, CompatBox},
            visitor::SyntaxVisitor,
        };

        static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

        fn get_compat() -> Compat {
            CONSTRUCTOR_COMPAT
                .get_or_init(|| {
                    from_str(include_str!($json_path)).unwrap()
                })
                .clone()
        }

        $(
            fn $walk_fn($ctx: &mut Context, $it: &$expr_type $(, $extra_args: $extra_types)*) {
                if $body {
                    $ctx
                        .usage
                        .push(CompatBox::new($it.span.clone(), get_compat()));
                }
            }
        )*

        pub fn $setup_fn($v: &mut SyntaxVisitor) {
            $setup_body
        }
    };
}
