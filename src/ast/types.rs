use derive_more::Display;

use super::*;

pub type TypePath = SimplePath;

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Type {
    #[display(fmt = "({})", _0)]
    Parenthesized(Box<Type>),
    #[display(fmt = "{}", _0)]
    Path(TypePath),
    #[display(fmt = "({}{})", _0, "if _0.0.len() == 1 { \",\" } else { \"\" }")]
    Tuple(Comma<Type>),
    #[display(fmt = "[{}; {}]", _0, _1)]
    Array(Box<Type>, Box<Lit>),
    #[display(fmt = "[{}]", _0)]
    Slice(Box<Type>),
    #[display(fmt ="_")]
    Infer,
    #[display(
        fmt = "fn({}){}",
        _0,
        "_1.as_ref().map(|x| format!(\" -> {}\", x)).unwrap_or_default()"
    )]
    Fn(Comma<Type>, Option<Box<Type>>),
    #[display(fmt = "{}", _0)]
    MacroInvocation(MacroInvocation),
}
