use derive_more::Display;

use std::fmt::{self, Debug, Display, Formatter};

macro_rules! punct {
    ($ident: ident, $punct:expr, $delimit_last: expr) => {
        #[derive(Clone, Debug, PartialEq, Default)]
        pub struct $ident<T: Clone + Display + Debug + PartialEq>(pub Vec<T>);

        impl<T: Clone + Display + Debug + PartialEq> Display for $ident<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                if !$delimit_last {
                    if let Some(item) = self.0.first() {
                        write!(f, "{}", item)?;
                    }
                    for item in self.0.iter().skip(1) {
                        write!(f, "{} {}", $punct, item)?;
                    }
                } else {
                    if let Some(item) = self.0.first() {
                        write!(f, "{}{}", item, $punct)?;
                    }
                    for item in self.0.iter().skip(1) {
                        write!(f, " {}{}", item, $punct)?;
                    }
                }
                Ok(())
            }
        }

        impl<T: Clone + Display + Debug + PartialEq> From<Vec<T>> for $ident<T> {
            fn from(v: Vec<T>) -> Self {
                Self(v)
            }
        }
    };
}

punct!(Comma, ',', false);
punct!(Add, '+', false);
punct!(Pipe, '|', false);
punct!(Semi, ';', true);

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Newline<T: Clone + Display + Debug + PartialEq>(pub Vec<T>);

impl<T: Clone + Display + Debug + PartialEq> Display for Newline<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for item in self.0.iter() {
            write!(f, "{}\n", item)?;
        }
        Ok(())
    }
}

mod expr;
mod item;
mod macaroni;
mod pat;
mod types;

pub use expr::*;
pub use item::*;
pub use macaroni::*;
pub use pat::*;
pub use types::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Block(pub Vec<Stmt>);

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for stmt in self.0.iter().take(self.0.len().saturating_sub(1)) {
            write!(f, "{} ", stmt)?;
        }
        if let Some(stmt) = self.0.last() {
            write!(f, "{} ", stmt)?;
        }
        write!(f, "}}")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
#[display(fmt = "{}", inner)]
pub struct Ident {
    pub inner: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SimplePath {
    pub leading_sep: Option<()>,
    pub segments: Vec<Ident>,
}

impl Display for SimplePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, seg) in self.segments.iter().enumerate() {
            write!(
                f,
                "{}{}",
                if i != 0 || self.leading_sep.is_some() {
                    "::"
                } else {
                    ""
                },
                seg
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Stmt {
    #[display(
        fmt = "let {} = {} {};",
        _0,
        "_1.as_ref().map(|x| format!(\": {}\", x)).unwrap_or_default()",
        "_2.as_ref().map(|x| format!(\"{}\", x)).unwrap_or_default()"
    )]
    Local(Pat, Option<Type>, Option<Expr>),
    #[display(fmt = "{}", _0)]
    Item(Item),
    #[display(fmt = "{}{}", _0, "if *_1 { \";\" } else { \"\" }")]
    Expr(Expr, bool),
    #[display(fmt = "{}", _0)]
    MacroInvocation(MacroInvocation),
}
