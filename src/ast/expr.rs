use derive_more::Display;
use rug::{Float, Integer as Int};

use std::str::FromStr;

use super::*;

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Expr {
    // Block(Block),
    #[display(fmt = "({} {})", _0, _1)]
    Unary(UnOp, Box<Expr>),
    #[display(fmt = "({} {} {})", _1, _0, _2)]
    Binary(Box<Expr>, BinOp, Box<Expr>),
    #[display(fmt = "({} {} {})", _1, _0, _2)]
    Assign(Box<Expr>, AssOp, Box<Expr>),
    #[display(fmt = "[{}; {}]", _0, _1)]
    Repeat(Box<Expr>, Box<Expr>),
    /// From, to, closed-ness
    #[display(
        fmt = "{}..{}{}",
        "_0.as_ref().map(|x| format!(\"{}\", x)).unwrap_or_default()",
        "if *_2 { \"=\" } else { \"\" }",
        "_1.as_ref().map(|x| format!(\"{}\", x)).unwrap_or_default()"
    )]
    Range(Option<Box<Expr>>, Option<Box<Expr>>, bool),
    #[display(fmt = "{}", _0)]
    Concat(Comma<Expr>),
    #[display(fmt = "{}", _0)]
    Path(Path),
    #[display(fmt = "{}", _0)]
    Lit(Lit),
    #[display(fmt = "{}.{}", _0, _1)]
    Field(Box<Expr>, Member),
    #[display(fmt = "{}({})", _0, _1)]
    Call(Box<Expr>, Comma<Expr>),
    #[display(fmt = "{}.{}({})", _0, _1, _2)]
    MethodCall(Box<Expr>, Ident, Comma<Expr>),
    #[display(fmt = "{}[{}]", _0, _1)]
    Index(Box<Expr>, Box<Expr>),
    #[display(fmt = "[{}]", _0)]
    Array(Comma<Expr>),
    #[display(fmt = "({})", _0)]
    Tuple(Comma<Expr>),
    #[display(fmt = "(as {} {})", _0, _1)]
    Cast(Box<Expr>, Box<Type>),
    #[display(
        fmt = "(if {} {}{})",
        _0,
        _1,
        "_2.as_ref().map(|x| format!(\" (else {})\", x)).unwrap_or_default()"
    )]
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    #[display(fmt = "(match {} {})", _0, _1)]
    Match(Box<Expr>, Comma<Arm>),
    #[display(fmt = "{}", _0)]
    Block(Block),
    #[display(
        fmt = "return {}",
        "_0.as_ref().map(|x| format!(\"{}\", x)).unwrap_or_default()"
    )]
    Return(Option<Box<Expr>>),
    #[display(fmt = "{}", _0)]
    Struct(Path, Comma<FieldValue>, Option<Box<Expr>>),
}

#[derive(Clone, Debug, PartialEq, Display)]
#[display(
    fmt = "(=> {}{} {})",
    pat,
    "guard.as_ref().map(|x| format!(\"(if {})\", x)).unwrap_or_default()",
    body
)]
pub struct Arm {
    pub pat: Pat,
    pub guard: Option<Expr>,
    pub body: Expr,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
pub enum Member {
    #[display(fmt = "{}", _0)]
    Named(Ident),
    #[display(fmt = "{}", _0)]
    Unnamed(Int),
}

#[derive(Clone, Debug, PartialEq, Display)]
#[display(fmt = "{}: {}", member, expr)]
pub struct FieldValue {
    pub member: Member,
    pub expr: Expr,
}

macro_rules! op_enum {
    ($name: ident { $($varname: ident => $varval: expr),+ }) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
        pub enum $name {
            $(
                #[display(fmt = $varval)]
                $varname,
            )+
        }

        impl $name {
            pub fn variants() -> Vec<Self> {
                vec![
                    $(
                        Self::$varname,
                    )+
                ]
            }
        }

        impl FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $varval => Ok(Self::$varname),
                    )+
                    _ => Err(()),
                }
            }
        }
    };
}

op_enum!(BinOp {
    Add => "+",
    Sub => "-",
    Mul => "*",
    Div => "/",
    Rem => "%",
    Exp => "**",
    // And => "&&",
    // Or => "||",
    BitXor => "^",
    BitAnd => "&",
    BitOr => "|",
    Shl => "<<",
    Shr => ">>",
    Eq => "==",
    Lt => "<",
    Le => "<=",
    Ne => "!=",
    Ge => ">=",
    Gt => ">"
});

op_enum!( UnOp {
    Not => "!",
    Neg => "-"
});

op_enum!(AssOp {
    Eq => "=",
    AddEq => "+=",
    SubEq => "-=",
    MulEq => "*=",
    DivEq => "/=",
    RemEq => "%=",
    ExpEq => "**=",
    BitXorEq => "^=",
    BitAndEq => "&=",
    BitOrEq => "|=",
    ShlEq => "<<=",
    ShrEq => ">>="
});

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Lit {
    #[display(fmt = "{}", val)]
    Int {
        val: Int,
        /// i.e. i32
        type_hint: Option<(char, usize)>,
    },
    #[display(fmt = "{}", val)]
    Float { val: Float },
}
