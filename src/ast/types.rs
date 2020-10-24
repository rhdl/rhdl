use super::*;

use std::boxed::Box;

crate::class_from_tokens! {
    Type {
        Parenthesized {
            paren_open: ParenOpen,
            inner : Box<Type>,
            paren_close: ParenClose
        },
        Path {
            inner: SimplePath
        },
        Tuple {
            paren_open: ParenOpen,
            inner: Punctuated<Type, Comma>,
            paren_close: ParenClose

        },
        Array {
            bracket_open: BracketOpen,
            ty: Box<Type>,
            semi: Semi,
            lit: Box<Lit>,
            bracket_close: BracketClose
        },
        Slice {
            bracket_open: BracketOpen,
            ty: Box<Type>,
            bracket_close: BracketClose
        },
        Infer {
            inner: Underscore
        },
        Fn {
            r#fn: Fn,
            paren_open: ParenOpen,
            args: Punctuated<Type, Comma>,
            paren_close: ParenClose,
            ret: Option<(RArrow, Box<Type>)>
        }
    }
}
