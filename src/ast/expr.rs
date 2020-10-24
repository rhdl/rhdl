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
            lower: Box<Expr>,
            range_type: RangeType,
            upper: Box<Expr>
        },
        Path {
            inner: SimplePath
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
            name: Ident,
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
            elements: Punctuated<Comma, Expr>,
            bracket_close: BracketClose
        },
        Tuple {
            paren_open: ParenOpen,
            elements: Punctuated<Comma, Expr>,
            paren_close: ParenClose
        },
        Cast {
            expr: Box<Expr>,
            r#as: As,
            ty: Box<Type>
        },
        For {
            r#for: For,
            pat: Box<Pat>,
            r#in: In,
            expr: Box<Expr>,
            body: Block
        },
        If {
            r#if: If,
            expr: Box<Expr>,
            body: Block,
            r#else: Option<(Else, Box<Expr>)>
        },
        Match {
            r#match: Match,
            expr: Box<Expr>,
            brace_open: BraceOpen,
            arms: Punctuated<Arm, Comma>,
            brace_close: BraceClose
        },
        Block {
            inner: Block
        },
        Return {
            r#return: Return,
            expr: Option<Box<Expr>>
        },
        Struct {
            path: ExprPath,
            brace_open: BraceOpen,
            fields: Punctuated<FieldValue, Comma>,
            brace_close: BraceClose
        }
    }
}

crate::inst_from_tokens! {
    Arm {
        pat: Pat,
        guard: Option<(If, Expr)>,
        fat_arrow: FatArrow,
        body: Expr
    },
    FieldValue {
        member: Member,
        colon: Colon,
        expr: Option<Expr>
    }
}

crate::class_from_tokens! {
    Member {
        Named {
            inner: Ident
        },
        Unnamed {
            // todo: lit into litint and litfloat
            inner: Lit
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
