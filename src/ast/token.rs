use super::*;

use derive_more::Display;
use rug::{Float, Integer as Int};

#[derive(Clone, Debug, PartialEq, Hash, Display)]
#[display(fmt = "{}:{}", _0, _1)]
pub struct Span(pub usize, pub usize);

impl std::ops::Add for Span {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0.min(rhs.0), self.1.max(rhs.1))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Display)]
#[display(fmt = "{}", inner)]
pub struct Ident {
    pub inner: String,
    pub span: Span,
}

impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Lit {
    #[display(fmt = "{}", raw)]
    Int {
        val: Int,
        /// i.e. i32
        suffix: Option<Ident>,
        raw: String,
        span: Span,
    },
    #[display(fmt = "{}", raw)]
    Float {
        val: Float,
        suffix: Option<Ident>,
        raw: String,
        span: Span,
    },
}

impl Spanned for Lit {
    fn span(&self) -> Span {
        match self {
            Self::Int { span, .. } | Self::Float { span, .. } => span.clone(),
        }
    }
}

pub trait ToTokens {
    fn to_tokens(&self) -> Vec<Tok>;
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl<T: ToTokens> Spanned for T {
    fn span(&self) -> Span {
        let tokens = self.to_tokens();
        let mut acc = tokens.first().unwrap().span();
        for token in tokens.iter().skip(1) {
            acc = acc + token.span();
        }
        acc
    }
}

macro_rules! token {
    ($variant: ident => $format: expr) => {
        #[derive(Debug, PartialEq, Display)]
        #[display(fmt = $format)]
        pub struct $variant {
            pub start: usize,
        }

        impl Spanned for $variant {
            fn span(&self) -> Span {
                Span(self.start, self.start + $format.len())
            }
        }
    };
    ($variant: ident) => {
        #[derive(Debug, PartialEq, Display)]
        #[display(fmt = stringify!($variant))]
        pub struct $variant {
            span: Span,
        }
        impl Spanned for $variant {
            fn span(&self) -> Span {
                self.span.clone()
            }
        }
    };
}

macro_rules! tokens {
    ($($variant: ident $( => $format: expr)?),*) => {
        $(
            token!($variant $(=> $format)?);
        )*
        #[derive(Debug, PartialEq, Display)]
        pub enum Tok {
            $(
                $variant($variant),
            )*
            Ident(Ident),
            Lit(Lit),
        }

        impl Spanned for Tok {
            fn span(&self) -> Span {
                match self {
                    $(
                        Self::$variant(x) => x.span(),
                    )*
                    Self::Ident(ident) => ident.span(),
                    Self::Lit(lit) => lit.span(),
                }
            }
        }
    };
}

tokens! {
    As,
    Break,
    Const,
    Continue,
    Crate,
    Else,
    Enum,
    Extern,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Match,
    Mod,
    Move,
    Mut,
    Pub,
    Ref,
    Return,
    LowerSelf => "self",
    UpperSelf => "Self",
    Static,
    Struct,
    Super,
    Trait,
    True,
    Type,
    Unsafe,
    Use,
    Where,
    While,

    Async,
    Await,
    Dyn,

    Abstract,
    Become,
    Box,
    Do,
    Final,
    Macro,
    Override,
    Priv,
    Typeof,
    Unsized,
    Virtual,
    Yield,
    Try,

    Union,

    Entity,
    Bag,
    Ring,

    Plus => "+",
    Minus => "-",
    Star => "*",
    StarStar => "**",
    Slash => "/",
    Percent => "%",
    Caret => "^",
    Not => "!",
    And => "&",
    Or => "|",
    AndAnd => "&&",
    OrOr => "||",
    Shl => "<<",
    Shr => ">>",
    PlusEq => "+=",
    MinusEq => "-=",
    StarEq => "*=",
    StarStarEq => "**=",
    SlashEq => "/=",
    PercentEq => "%=",
    CaretEq => "^=",
    AndEq => "&=",
    OrEq => "|=",
    ShlEq => "<<=",
    ShrEq => ">>=",
    Eq => "=",
    EqEq => "==",
    Ne => "!=",
    Gt => ">",
    Lt => "<",
    Ge => ">=",
    Le => "<=",
    At => "@",
    Underscore => "_",
    Dot => ".",
    DotDot => "..",
    DotDotEq => "..=",
    Comma => ",",
    Semi => ";",
    Colon => ":",
    PathSep => "::",
    RArrow => "->",
    FatArrow => "=>",
    Pound => "#",
    Dollar => "$",
    Question => "?",
    BracketOpen => "[",
    BracketClose => "]",
    ParenOpen => "(",
    ParenClose => ")",
    BraceOpen => "{{",
    BraceClose => "}}"
}
