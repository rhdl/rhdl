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
punct!(Implicit, "", false);
punct!(Semi, ";", true);

mod expr;
mod item;
mod pat;

pub use expr::*;
pub use item::*;
pub use pat::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Block(pub Vec<Stmt>);

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for stmt in self.0.iter().take(self.0.len().saturating_sub(1)) {
            write!(f, "{} ", stmt)?;
        }
        if let Some(stmt) = self.0.last() {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Type {
    #[display(fmt = "[{}; {}]", _0, _1)]
    Array(Box<Type>, Box<Lit>),
    #[display(fmt = "{}", _0)]
    Path(Path),
    #[display(
        fmt = "fn({}){}",
        _0,
        "_1.as_ref().map(|x| format!(\" -> ({})\", x)).unwrap_or_default()"
    )]
    Fn(Comma<Type>, Option<Box<Type>>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
#[display(fmt = "{}", inner)]
pub struct Ident {
    pub inner: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Path {
    pub leading_colon: Option<()>,
    pub segments: Vec<Ident>,
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, seg) in self.segments.iter().enumerate() {
            write!(
                f,
                "{}{}",
                if i != 0 || self.leading_colon.is_some() {
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
        fmt = "(let {}{} {});",
        _0,
        "_1.as_ref().map(|x| format!(\": {}\", x)).unwrap_or_default()",
        "_2.as_ref().map(|x| format!(\"{}\", x)).unwrap_or_default()"
    )]
    Local(Pat, Option<Type>, Option<Expr>),
    #[display(fmt = "{}", _0)]
    Item(Item),
    #[display(fmt = "{}{}", _0, "if *_1 { \";\" } else { \"\" }")]
    Expr(Expr, bool),
}
