use paste::paste;
use std::boxed::Box;

use super::*;

crate::class_from_tokens! {
    Expr {
        Unary {
            op: UnOp,
            expr: Box<Expr>
        },
        Binary {
            left: Box<Expr>,
            op: BinOp,
            right: Box<Expr>
        },
        Assign {
            lhs: Box<Expr>,
            op: AssOp,
            rhs: Box<Expr>
        },
        Repeat {
            bracket_open: BracketOpen,
            init: Box<Expr>,
            semi: Semi,
            repeat: Box<Expr>,
            bracket_close: BracketClose
        },
        Range {
            left: Option<Box<Expr>>,
            range_type: RangeType,
            right: Option<Box<Expr>>
        },
        Path {
            leading_sep: Option<PathSep>,
            segments: Punctuated<PathSegment, PathSep>
        },
        Lit {
            inner: Lit
        },
        Field {
            on: Box<Expr>,
            dot: Dot,
            member: Member
        },
        Call {
            on: Box<Expr>,
            paren_open: ParenOpen,
            args: Punctuated<Expr, Comma>,
            paren_close: ParenClose
        },
        MethodCall {
            on: Box<Expr>,
            dot: Dot,
            method: Ident,
            paren_open: ParenOpen,
            args: Punctuated<Expr, Comma>,
            paren_close: ParenClose
        },
        Index {
            on: Box<Expr>,
            bracket_open: BracketOpen,
            index: Box<Expr>,
            bracket_close: BracketClose
        },
        Array {
            bracket_open: BracketOpen,
            elements: Punctuated<Expr, Comma>,
            bracket_close: BracketClose
        },
        Tuple {
            paren_open: ParenOpen,
            elements: Punctuated<Expr, Comma>,
            paren_close: ParenClose
        },
        Cast {
            expr: Box<Expr>,
            as_token: As,
            ty: Box<Type>
        },
        For {
            for_token: For,
            pat: Box<Pat>,
            in_token: In,
            expr: Box<Expr>,
            block: Block
        },
        If {
            if_token: If,
            expr: Box<Expr>,
            block: Block,
            else_token: Option<(Else, Box<Expr>)>
        },
        Match {
            match_token: Match,
            expr: Box<Expr>,
            brace_open: BraceOpen,
            arms: Punctuated<Arm, Comma>,
            brace_close: BraceClose
        },
        Block {
            inner: Block
        },
        Return {
            return_token: Return,
            expr: Option<Box<Expr>>
        },
        Struct {
            path: ExprPath,
            brace_open: BraceOpen,
            fields: Punctuated<FieldValue, Comma>,
            base: Option<(DotDot, Box<Expr>)>,
            comma: Option<Comma>,
            brace_close: BraceClose
        },
        Grouped {
            paren_open: ParenOpen,
            expr: Box<Expr>,
            paren_close: ParenClose
        }
    }
}

crate::insts_from_tokens! {
    PathSegment {
        ident: Ident,
        generic_args: Option<GenericArgs>
    },
    Arm {
        pat: Pat,
        guard: Option<(If, Expr)>,
        fat_arrow: FatArrow,
        body: Expr
    },
    FieldValue {
        member: Member,
        expr: Option<(Colon, Expr)>
    },
    GenericArgs {
        path_sep: Option<PathSep>,
        lt: Lt,
        args: Punctuated<GenericArg, Comma>,
        gt: Gt
    }
}

crate::class_from_tokens! {
    GenericArg {
        Type {
            inner: Type
        },
        Expr {
            inner: Expr
        },
        Binding {
            ident: Ident,
            eq: Eq,
            ty: Type
        }
    }
}

crate::class_from_tokens! {
    Member {
        Named {
            inner: Ident
        },
        Unnamed {
            inner: LitInt
        }
    }
}

crate::class_only_from_tokens! {
    BinOp {
        Plus,
        Minus,
        Star,
        Slash,
        Percent,
        StarStar,
        Caret,
        And,
        Or,
        Shl,
        Shr,
        EqEq,
        Lt,
        Le,
        Ne,
        Ge,
        Gt
    }
}

crate::class_only_from_tokens! {
    UnOp {
        Not,
        Minus
    }
}

crate::class_only_from_tokens! {
    AssOp {
        Eq,
        PlusEq,
        MinusEq,
        StarEq,
        SlashEq,
        PercentEq,
        StarStarEq,
        CaretEq,
        AndEq,
        OrEq,
        ShlEq,
        ShrEq
    }
}
