use super::*;

use derive_more::Display;
use rug::{Float, Integer as Int};

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Span(pub usize, pub usize);

impl std::ops::Add for Span {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0.min(rhs.0), self.1.max(rhs.1))
    }
}

pub trait ToTokens {
    fn to_tokens(&self) -> Vec<Tok>;
    fn first(&self) -> Tok;
    fn last(&self) -> Tok;
    fn len(&self) -> usize;
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl<T: ToTokens> Spanned for T {
    fn span(&self) -> Span {
        self.first().span() + self.last().span()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Display)]
#[display(fmt = "{}", inner)]
pub struct Ident {
    pub inner: String,
    pub span: Span,
}

impl ToTokens for Ident {
    fn to_tokens(&self) -> Vec<Tok> {
        vec![Tok::Ident(self.clone())]
    }

    fn first(&self) -> Tok {
        Tok::Ident(self.clone())
    }

    fn last(&self) -> Tok {
        Tok::Ident(self.clone())
    }

    fn len(&self) -> usize {
        1
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

impl ToTokens for Lit {
    fn to_tokens(&self) -> Vec<Tok> {
        vec![Tok::Lit(self.clone())]
    }

    fn first(&self) -> Tok {
        Tok::Lit(self.clone())
    }

    fn last(&self) -> Tok {
        Tok::Lit(self.clone())
    }

    fn len(&self) -> usize {
        1
    }
}

macro_rules! token {
    ($format: literal => $variant: ident) => {
        #[derive(Debug, Hash, Clone, PartialEq, Display)]
        #[display(fmt = $format)]
        pub struct $variant {
            pub start: usize,
        }

        impl ToTokens for $variant {
            fn to_tokens(&self) -> Vec<Tok> {
                vec![Tok::$variant(self.clone())]
            }

            fn first(&self) -> Tok {
                Tok::$variant(self.clone())
            }

            fn last(&self) -> Tok {
                Tok::$variant(self.clone())
            }

            fn len(&self) -> usize {
                1
            }
        }
    };
    ($variant: ident) => {
        #[derive(Debug, Clone, Hash, PartialEq, Display)]
        #[display(fmt = stringify!($variant))]
        pub struct $variant {
            pub start: usize,
        }

        impl ToTokens for $variant {
            fn to_tokens(&self) -> Vec<Tok> {
                vec![Tok::$variant(self.clone())]
            }

            fn first(&self) -> Tok {
                Tok::$variant(self.clone())
            }

            fn last(&self) -> Tok {
                Tok::$variant(self.clone())
            }

            fn len(&self) -> usize {
                1
            }
        }
    };
}

macro_rules! tokens {
    ($($($format: literal =>)? $variant: ident),*) => {
        $(
            token!($($format =>)? $variant);
        )*
        #[derive(Debug, Clone, PartialEq, Display)]
        pub enum Tok {
            $( $variant($variant) ),*,
            Ident(Ident),
            Lit(Lit),
        }

        impl Spanned for Tok {
            fn span(&self) -> Span {
                match self {
                    $( Self::$variant(x) => x.span() ),*,
                    Self::Ident(Ident { span, ..}) => span.clone(),
                    Self::Lit(lit) => match lit {
                        Lit::Int { span, .. } | Lit::Float { span, .. } => span.clone(),
                    },
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
    "self" => LowerSelf,
    "Self" => UpperSelf,
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

    "+" => Plus,
    "-" => Minus,
    "*" => Star,
    "**" => StarStar,
    "/" => Slash,
    "%" => Percent,
    "^" => Caret,
    "!" => Not,
    "&" => And,
    "|" => Or,
    "&&" => AndAnd,
    "||" => OrOr,
    "<<" => Shl,
    ">>" => Shr,
    "+=" => PlusEq,
    "-=" => MinusEq,
    "*=" => StarEq,
    "**=" => StarStarEq,
    "/=" => SlashEq,
    "%=" => PercentEq,
    "^=" => CaretEq,
    "&=" => AndEq,
    "|=" => OrEq,
    "<<=" => ShlEq,
    ">>=" => ShrEq,
    "=" => Eq,
    "==" => EqEq,
    "!=" => Ne,
    ">" => Gt,
    "<" => Lt,
    ">=" => Ge,
    "<=" => Le,
    "@" => At,
    "_" => Underscore,
    "." => Dot,
    ".." => DotDot,
    "..=" => DotDotEq,
    "," => Comma,
    ";" => Semi,
    ":" => Colon,
    "::" => PathSep,
    "->" => RArrow,
    "=>" => FatArrow,
    "#" => Pound,
    "$" => Dollar,
    "?" => Question,
    "[" => BracketOpen,
    "]" => BracketClose,
    "(" => ParenOpen,
    ")" => ParenClose,
    "{{" => BraceOpen,
    "}}" => BraceClose
}
