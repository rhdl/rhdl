use super::*;

use std::boxed::Box;

// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum Type {
//     #[display(fmt = "({})", _0)]
//     Parenthesized(Box<Type>),
//     #[display(fmt = "{}", _0)]
//     Path(TypePath),
//     #[display(fmt = "({}{})", _0, "if _0.0.len() == 1 { \",\" } else { \"\" }")]
//     Tuple(Comma<Type>),
//     #[display(fmt = "[{}; {}]", _0, _1)]
//     Array(Box<Type>, Box<Lit>),
//     #[display(fmt = "[{}]", _0)]
//     Slice(Box<Type>),
//     #[display(fmt = "_")]
//     Infer,
//     #[display(
//         fmt = "fn({}){}",
//         _0,
//         "_1.as_ref().map(|x| format!(\" -> {}\", x)).unwrap_or_default()"
//     )]
//     Fn(Comma<Type>, Option<Box<Type>>),
//     #[display(fmt = "{}", _0)]
//     MacroInvocation(MacroInvocation),
// }

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
