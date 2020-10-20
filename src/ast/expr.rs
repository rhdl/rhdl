use derive_more::Display;
use rug::Integer as Int;

use std::boxed::Box;
use std::str::FromStr;

use super::*;

// pub type ExprPath = SimplePath;
// pub type ExprPathSegment = Ident;

// #[derive(Clone, Debug, PartialEq, Display)]
// #[display(fmt = "{} = {}", ident, ty)]
// pub struct GenericArgsBinding {
//     pub ident: Ident,
//     pub ty: Box<Type>,
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// #[display(fmt = "{},{}", tys, bindings)]
// pub struct GenericArgs {
//     pub tys: Comma<Type>,
//     pub bindings: Comma<GenericArgsBinding>,
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// #[display(
//     fmt = "{}{}",
//     ident,
//     "args.as_ref().map(|x| format!(\"::<{}>\", x)).unwrap_or_default()"
// )]
// pub struct ExprPathSegment {
//     pub ident: Ident,
//     pub args: Option<GenericArgs>,
// }

// #[derive(Clone, Debug, PartialEq)]
// pub struct ExprPath {
//     pub leading_colon: Option<()>,
//     pub segments: Vec<ExprPathSegment>,
//     pub qself: Option<QSelf>,
// }

// impl Display for ExprPath {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         if let Some(qself) = self.qself.as_ref() {
//             write!(f, "<{}>", qself)?;
//         }
//         for (i, seg) in self.segments.iter().enumerate() {
//             write!(
//                 f,
//                 "{}{}",
//                 if i != 0 || self.leading_colon.is_some() {
//                     "::"
//                 } else {
//                     ""
//                 },
//                 seg
//             )?;
//         }
//         Ok(())
//     }
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// #[display(
//     fmt = "{}{}",
//     ty,
//     "path.as_ref().map(|x| format!(\"as {}\", x)).unwrap_or_default()"
// )]
// pub struct QSelf {
//     pub ty: Box<Type>,
//     pub path: Option<Box<TypePath>>,
// }
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
            pat: Box<Pat>,
            r#in: In,
            expr: Box<Expr>,
            body: Block
        },
        If {
            expr: Box<Expr>,
            body: Block,
            r#else: Option<Box<Expr>>
        },
        Match {
            expr: Box<Expr>,
            arms: Punctuated<Arm, Comma>
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
        guard: Option<Expr>,
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

// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum Expr {
//     // Block(Block),
//     #[display(fmt = "{}{}", _0, _1)]
//     Unary(UnOp, Box<Expr>),
//     #[display(fmt = "{} {} {}", _0, _1, _2)]
//     Binary(Box<Expr>, BinOp, Box<Expr>),
//     #[display(fmt = "{} {} {}", _0, _1, _2)]
//     Assign(Box<Expr>, AssOp, Box<Expr>),
//     #[display(fmt = "[{}; {}]", _0, _1)]
//     Repeat(Box<Expr>, Box<Expr>),
//     /// From, to, closed-ness
//     #[display(
//         fmt = "{}..{}{}",
//         "_0.as_ref().map(|x| format!(\"{}\", x)).unwrap_or_default()",
//         "if *_2 { \"=\" } else { \"\" }",
//         "_1.as_ref().map(|x| format!(\"{}\", x)).unwrap_or_default()"
//     )]
//     Range(Option<Box<Expr>>, Option<Box<Expr>>, bool),
//     // #[display(fmt = "{}", _0)]
//     // Concat(Comma<Expr>),
//     #[display(fmt = "{}", _0)]
//     Path(ExprPath),
//     #[display(fmt = "{}", _0)]
//     Lit(Lit),
//     #[display(fmt = "{}.{}", _0, _1)]
//     Field(Box<Expr>, Member),
//     #[display(fmt = "{}({})", _0, _1)]
//     Call(Box<Expr>, Comma<Expr>),
//     #[display(fmt = "{}.{}({})", _0, _1, _2)]
//     MethodCall(Box<Expr>, ExprPathSegment, Comma<Expr>),
//     #[display(fmt = "{}[{}]", _0, _1)]
//     Index(Box<Expr>, Box<Expr>),
//     #[display(fmt = "[{}]", _0)]
//     Array(Comma<Expr>),
//     #[display(fmt = "({})", _0)]
//     Tuple(Comma<Expr>),
//     #[display(fmt = "{} as {}", _0, _1)]
//     Cast(Box<Expr>, Box<Type>),
//     #[display(fmt = "for {} in {} {}", _0, _1, _2)]
//     For(Box<Pat>, Box<Expr>, Block),
//     #[display(
//         fmt = "if {} {}{}",
//         _0,
//         _1,
//         "_2.as_ref().map(|x| format!(\" else {}\", x)).unwrap_or_default()"
//     )]
//     If(Box<Expr>, Block, Option<Box<Expr>>),
//     #[display(
//         fmt = "match {} {{ {}{}}}",
//         _0,
//         _1,
//         "if _1.0.is_empty() { \"\" } else { \" \" }"
//     )]
//     Match(Box<Expr>, Comma<Arm>),
//     #[display(fmt = "{}", _0)]
//     Block(Block),
//     #[display(
//         fmt = "return {}",
//         "_0.as_ref().map(|x| format!(\"{}\", x)).unwrap_or_default()"
//     )]
//     Return(Option<Box<Expr>>),
//     #[display(
//         fmt = "{} {{ {}{}{}}}",
//         _0,
//         _1,
//         "_2.as_ref().map(|x| format!(\"{}.. {}\", if _1.0.is_empty() { \"\" } else { \", \" }, x)).unwrap_or_default()",
//         "if _1.0.is_empty() && _2.is_none() { \"\" } else { \" \" }"
//     )]
//     Struct(ExprPath, Comma<FieldValue>, Option<Box<Expr>>),
//     #[display(fmt = "{}", _0)]
//     MacroInvocation(MacroInvocation),
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// #[display(
//     fmt = "{}{} => {}",
//     pat,
//     "guard.as_ref().map(|x| format!(\" if {}\", x)).unwrap_or_default()",
//     body
// )]
// pub struct Arm {
//     pub pat: Pat,
//     pub guard: Option<Expr>,
//     pub body: Expr,
// }

// #[derive(Clone, Debug, Hash, PartialEq, Display)]
// pub enum Member {
//     #[display(fmt = "{}", _0)]
//     Named(Ident),
//     #[display(fmt = "{}", _0)]
//     Unnamed(Int),
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// #[display(
//     fmt = "{}{}",
//     member,
//     "expr.as_ref().map(|x| format!(\": {}\", x)).unwrap_or_default()"
// )]
// pub struct FieldValue {
//     pub member: Member,
//     pub expr: Option<Expr>,
// }

// macro_rules! op_enum {
//     ($name: ident { $($varname: ident => $varval: expr),+ }) => {
//         #[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
//         pub enum $name {
//             $(
//                 #[display(fmt = $varval)]
//                 $varname,
//             )+
//         }

//         impl $name {
//             pub fn variants() -> Vec<Self> {
//                 vec![
//                     $(
//                         Self::$varname,
//                     )+
//                 ]
//             }
//         }

//         impl FromStr for $name {
//             type Err = ();

//             fn from_str(s: &str) -> Result<Self, Self::Err> {
//                 match s {
//                     $(
//                         $varval => Ok(Self::$varname),
//                     )+
//                     _ => Err(()),
//                 }
//             }
//         }
//     };
// }

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
