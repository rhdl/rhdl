use derive_more::Display;
use rug::Integer;

use super::*;

#[derive(Clone, Debug, PartialEq, Display)]
#[display(fmt = "{}: {}", pat, ty)]
pub struct PatType {
    pub pat: Pat,
    pub ty: Type,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum StructPatternField {
    #[display(fmt = "{}: {}", _0, _1)]
    TuplePat(Integer, Box<Pat>),
    #[display(fmt = "{}: {}", _0, _1)]
    IdentPat(Ident, Box<Pat>),
    #[display(fmt = "{}", _0)]
    Ident(Ident),
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Pat {
    #[display(fmt = "{}", _0)]
    Lit(Lit),
    #[display(fmt = "{}", _0)]
    Ident(Ident),
    #[display(fmt = "{}", _0)]
    Path(ExprPath),
    #[display(fmt = "_")]
    Wildcard,
    #[display(fmt = "{}{}{}", _0, "if *_1 { \"..=\"} else { \"..\" }", _2)]
    Range(Expr, bool, Expr),
    #[display(
        fmt = "{}{}{}",
        _0,
        _1,
        "_2.as_ref().map(|x| \",..\".to_string()).unwrap_or_default()"
    )]
    Struct(ExprPath, Comma<StructPatternField>, Option<()>),
    #[display(
        fmt = "{}{}{}",
        _0,
        _1,
        "_2.as_ref().map(|x| format!(\", ..,{}\", x)).unwrap_or_default()"
    )]
    TupleStruct(ExprPath, Comma<Pat>, Option<Comma<Pat>>),
    #[display(
        fmt = "{}{}",
        _0,
        "_1.as_ref().map(|x| format!(\"..{}\", x)).unwrap_or_default()"
    )]
    Tuple(Comma<Pat>, Option<Comma<Pat>>),
    #[display(fmt = "[{}]", _0)]
    Slice(Comma<Pat>),
}
