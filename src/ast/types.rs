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
            leading_sep: Option<PathSep>,
            segments: Punctuated<TypePathSegment, PathSep>
        },
        Tuple {
            paren_open: ParenOpen,
            tys: Punctuated<Type, Comma>,
            paren_close: ParenClose

        },
        Array {
            bracket_open: BracketOpen,
            ty: Box<Type>,
            semi: Semi,
            lit: LitInt,
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
            fn_token: Fn,
            paren_open: ParenOpen,
            args: Punctuated<Type, Comma>,
            paren_close: ParenClose,
            ret: Option<(RArrow, Box<Type>)>
        }
    }
}


crate::insts_from_tokens! {
    TypePathSegment {
        ident: Ident,
        sep: Option<PathSep>,
        generics: Option<TypePathSegmentGenerics>
    }
}

crate::class_from_tokens! {
    TypePathSegmentGenerics {
        Generics {
            inner: Generics
        },
        Fn {
            paren_open: ParenOpen,
            inputs: Option<(ParenOpen, Punctuated<Type, Comma>, ParenClose)>,
            paren_close: ParenClose,
            ret: Option<(RArrow, Type)>
        }
    }
}
