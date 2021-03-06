use paste::paste;
use rug::{Float, Integer as Int};

use std::cmp::PartialEq;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Span(pub usize, pub usize);

impl std::ops::Add for Span {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0.min(rhs.0), self.1.max(rhs.1))
    }
}

impl std::ops::AddAssign for Span {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            0: self.0.min(rhs.0),
            1: self.1.max(rhs.1),
        }
    }
}

impl Into<std::ops::Range<usize>> for Span {
    fn into(self) -> std::ops::Range<usize> {
        self.0..self.1
    }
}

pub trait ToTokens: Clone {
    fn to_tokens(&self) -> Vec<Tok>;

    fn first(&self) -> Option<Tok> {
        self.to_tokens().first().cloned()
    }

    fn last(&self) -> Option<Tok> {
        self.to_tokens().last().cloned()
    }

    /// Number of contained tokens
    fn len(&self) -> usize;
}

impl<T: ToTokens> ToTokens for Vec<T> {
    fn to_tokens(&self) -> Vec<Tok> {
        self.iter().map(ToTokens::to_tokens).flatten().collect()
    }

    /// Number of contained tokens
    fn len(&self) -> usize {
        self.iter().map(ToTokens::len).sum()
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl<T: ToTokens> Spanned for T {
    fn span(&self) -> Span {
        if self.len() == 1 {
            self.first().map(|x| x.span()).unwrap_or(Span(0, 0))
        } else if self.len() == 0 {
            Span(0, 0)
        } else {
            self.first().map(|x| x.span()).unwrap() + self.last().map(|x| x.span()).unwrap()
        }
    }
}

#[derive(Clone, Debug, Eq)]
/// Equality is ONLY on inner, not on the span
pub struct Ident {
    pub inner: String,
    pub span: Span,
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::hash::Hash for Ident {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<'a> PartialEq<&'a str> for Ident {
    fn eq(&self, other: &&'a str) -> bool {
        self.inner == *other
    }
}

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool {
        self.inner == other
    }
}

impl PartialEq<Ident> for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self) -> Vec<Tok> {
        vec![Tok::Ident(self.clone())]
    }

    fn len(&self) -> usize {
        1
    }
}

pub(crate) fn visit_ident<'ast, V>(v: &mut V, inst: &'ast Ident)
where
    V: crate::visit::Visit<'ast> + ?Sized,
{
}

#[derive(Clone, Debug, PartialEq)]
pub enum Lit {
    Int(LitInt),
    Float(LitFloat),
    Bool(LitBool),
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(float) => write!(f, "{}", float),
            Self::Bool(b) => write!(f, "{}", b)
        }
    }
}

impl ToTokens for Lit {
    fn to_tokens(&self) -> Vec<Tok> {
        vec![Tok::Lit(self.clone())]
    }

    fn len(&self) -> usize {
        1
    }
}

pub(crate) fn visit_lit<'ast, V>(v: &mut V, inst: &'ast Lit)
where
    V: crate::visit::Visit<'ast> + ?Sized,
{
    match inst {
        Lit::Int(lit_int) => v.visit_lit_int(lit_int),
        Lit::Float(lit_float) => v.visit_lit_float(lit_float),
        Lit::Bool(lit_bool) => v.visit_lit_bool(lit_bool),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LitInt {
    pub val: Int,
    pub suffix: Option<Ident>,
    pub raw: String,
    pub span: Span,
}

impl fmt::Display for LitInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl ToTokens for LitInt {
    fn to_tokens(&self) -> Vec<Tok> {
        vec![Tok::Lit(Lit::Int(self.clone()))]
    }

    fn len(&self) -> usize {
        1
    }
}

pub(crate) fn visit_lit_int<'ast, V>(_v: &mut V, _inst: &'ast LitInt)
where
    V: crate::visit::Visit<'ast> + ?Sized,
{
}

#[derive(Clone, Debug, PartialEq)]
pub struct LitFloat {
    pub val: Float,
    pub suffix: Option<Ident>,
    pub raw: String,
    pub span: Span,
}

impl fmt::Display for LitFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl ToTokens for LitFloat {
    fn to_tokens(&self) -> Vec<Tok> {
        vec![Tok::Lit(Lit::Float(self.clone()))]
    }

    fn len(&self) -> usize {
        1
    }
}

pub(crate) fn visit_lit_float<'ast, V>(_v: &mut V, _inst: &'ast LitFloat)
where
    V: crate::visit::Visit<'ast> + ?Sized,
{
}

#[derive(Clone, Debug, PartialEq)]
pub struct LitBool {
    pub inner: bool,
    pub span: Span,
}

impl fmt::Display for LitBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl ToTokens for LitBool {
    fn to_tokens(&self) -> Vec<Tok> {
        vec![Tok::Lit(Lit::Bool(self.clone()))]
    }

    fn len(&self) -> usize {
        1
    }
}

pub(crate) fn visit_lit_bool<'ast, V>(_v: &mut V, _inst: &'ast LitBool)
where
    V: crate::visit::Visit<'ast> + ?Sized,
{
}

macro_rules! token {
    ($format: literal => $variant: ident) => {
        #[derive(Debug, Hash, Clone, PartialEq)]
        pub struct $variant {
            pub left: usize,
        }

        paste! {
            pub(crate) fn [<visit_ $variant:snake>]<'ast, V>(v: &mut V, inst: &'ast $variant) where V: crate::visit::Visit<'ast> + ?Sized { }
        }

        impl $variant {
            fn len() -> usize {
                $format.len()
            }
        }

        impl fmt::Display for $variant {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $format)
            }
        }

        impl ToTokens for $variant {
            fn to_tokens(&self) -> Vec<Tok> {
                vec![Tok::$variant(self.clone())]
            }

            fn len(&self) -> usize {
                1
            }
        }
    };
    ($variant: ident) => {
        #[derive(Debug, Clone, Hash, PartialEq)]
        pub struct $variant {
            pub left: usize,
        }

        paste! {
            pub(crate) fn [<visit_ $variant:snake>]<'ast, V>(v: &mut V, inst: &'ast $variant) where V: crate::visit::Visit<'ast> + ?Sized { }
        }

        paste::paste! {
            impl fmt::Display for $variant {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, stringify!([<$variant:lower>]))
                }
            }
        }

        impl $variant {
            fn len() -> usize {
                stringify!($variant).len()
            }
        }

        impl ToTokens for $variant {
            fn to_tokens(&self) -> Vec<Tok> {
                vec![Tok::$variant(self.clone())]
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
        #[derive(Debug, Clone, PartialEq)]
        pub enum Tok {
            $( $variant($variant) ),*,
            Ident(Ident),
            Lit(Lit),
        }

        impl fmt::Display for Tok {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$variant(v) => write!(f, "{}", v)),*,
                    Self::Ident(i) => write!(f, "{}", i),
                    Self::Lit(l) => write!(f, "{}", l)
                }
            }
        }

        impl Spanned for Tok {
            fn span(&self) -> Span {
                match self {
                    $( Self::$variant($variant { left }) => Span(*left, *left + $variant::len()) ),*,
                    Self::Ident(Ident { span, ..}) => span.clone(),
                    Self::Lit(lit) => match lit {
                        Lit::Int(LitInt { span, .. }) | Lit::Float(LitFloat { span, .. }) | Lit::Bool(LitBool { span, .. }) => span.clone(),
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
    "type" => TokenType,
    Unsafe,
    Use,
    Where,
    While,

    Async,
    Await,
    Dyn,

    Abstract,
    Become,
    "box" => TokenBox,
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
    Arch,
    When,
    Out,
    InOut,

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
