use std::boxed::Box;

use super::*;

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
