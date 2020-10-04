use derive_more::Display;

use super::*;

#[derive(Clone, Debug, PartialEq, Display)]
#[display(fmt = "{}: {}", pat, ty)]
pub struct PatType {
    pub pat: Box<Pat>,
    pub ty: Type,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Pat {
    #[display(fmt = "{}", _0)]
    Ident(Ident),
    #[display(fmt = "{}", _0)]
    Lit(Lit),
    #[display(fmt = "{}", _0)]
    Type(PatType),
}
