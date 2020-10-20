use std::boxed::Box;

use super::*;

// #[derive(Clone, Debug, PartialEq, Display)]
// #[display(fmt = "{}: {}", pat, ty)]
// pub struct PatType {
//     pub pat: Pat,
//     pub ty: Type,
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum StructPatternField {
//     #[display(fmt = "{}: {}", _0, _1)]
//     TuplePat(Integer, Box<Pat>),
//     #[display(fmt = "{}: {}", _0, _1)]
//     IdentPat(Ident, Box<Pat>),
//     #[display(fmt = "{}", _0)]
//     Ident(Ident),
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum Pat {
//     #[display(fmt = "{}", _0)]
//     Lit(Lit),
//     #[display(fmt = "{}", _0)]
//     Ident(Ident),
//     #[display(fmt = "{}", _0)]
//     Path(ExprPath),
//     #[display(fmt = "_")]
//     Wildcard,
//     #[display(fmt = "{}{}{}", _0, "if *_1 { \"..=\"} else { \"..\" }", _2)]
//     Range(Expr, bool, Expr),
//     #[display(
//         fmt = "{} {{ {}{}{}{}}}",
//         _0,
//         _1,
//         "if !_1.0.is_empty() && _2.is_some() { \", \" } else { \"\" }",
//         "if _2.is_some() { \"..\" } else { \"\" }",
//         "if _1.0.is_empty() && _2.is_none() { \"\" } else { \" \" }"
//     )]
//     Struct(ExprPath, Comma<StructPatternField>, Option<()>),
//     #[display(
//         fmt = "{}({}{})",
//         _0,
//         _1,
//         "_2.as_ref().map(|x| format!(\", .., {}\", x)).unwrap_or_default()"
//     )]
//     TupleStruct(ExprPath, Comma<Pat>, Option<Comma<Pat>>),
//     #[display(
//         fmt = "({}{})",
//         _0,
//         "_1.as_ref().map(|x| format!(\", .., {}\", x)).unwrap_or_default()"
//     )]
//     Tuple(Comma<Pat>, Option<Comma<Pat>>),
//     #[display(fmt = "[{}]", _0)]
//     Slice(Comma<Pat>),
// }

crate::inst_from_tokens! {
    PatType {
        pat: Pat,
        colon: Colon,
        ty: Type
    }
}

crate::class_from_tokens! {
    StructPatternField {
        TuplePat {
            // TODO: make lit into litint + litfloat
            index: Lit,
            colon: Colon,
            pat: Box<Pat>
        },
        IdentPat {
            ident: Ident,
            colon: Colon,
            pat: Box<Pat>
        },
        Ident {
            ident: Ident
        }
    }
}

crate::class_from_tokens! {
    Pat {
        Lit {
            inner: Lit
        },
        Path {
            inner: ExprPath
        },
        Wildcard {
            inner: Star
        },
        Range {
            lower: Expr,
            range_type: RangeType,
            upper: Expr
        },
        Struct {
            path: ExprPath,
            brace_open: BraceOpen,
            fields: Option<Punctuated<StructPatternField, Comma>>,
            remaining: Option<DotDot>,
            brace_close: BraceClose
        },
        TupleStruct {
            path: ExprPath,
            paren_open: ParenOpen,
            subpats: Punctuated<Pat, Comma>,
            rest_subpats: Option<(DotDot, Punctuated<Pat, Comma>)>,
            paren_close: ParenClose
        },
        Tuple {
            paren_open: ParenOpen,
            subpats: Punctuated<Pat, Comma>,
            rest_subpats: Option<(DotDot, Punctuated<Pat, Comma>)>,
            paren_close: ParenClose
        },
        Slice {
            bracket_open: BracketOpen,
            subpats: Punctuated<Pat, Comma>,
            bracket_close: BracketClose
        }
    }
}
