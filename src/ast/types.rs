use derive_more::Display;

use super::*;

pub type TypePath = ExprPath;

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Type {
    #[display(fmt = "[{}; {}]", _0, _1)]
    Array(Box<Type>, Box<Lit>),
    #[display(fmt = "{}", _0)]
    Path(TypePath),
    #[display(
        fmt = "fn({}){}",
        _0,
        "_1.as_ref().map(|x| format!(\" -> ({})\", x)).unwrap_or_default()"
    )]
    Fn(Comma<Type>, Option<Box<Type>>),
}
