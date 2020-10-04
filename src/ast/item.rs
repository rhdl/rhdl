use derive_more::Display;

use std::fmt::Debug;

use super::*;

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Vis {
    #[display(fmt = "pub ")]
    Pub,
    #[display(fmt = "crate ")]
    Crate,
    #[display(fmt = "pub({}) ", _0)]
    Restricted(Path),
    #[display(fmt = "")]
    Inherited,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum GenericParam {
    #[display(
        fmt = "{}: {}{}",
        _0,
        _1,
        "_2.as_ref().map(|x| format!(\" = {}\", x)).unwrap_or_default()"
    )]
    Type(Ident, Add<Path>, Option<Type>),
    #[display(
        fmt = "const {}: {}{}",
        _0,
        _1,
        "_2.as_ref().map(|x| format!(\" = {}\", x)).unwrap_or_default()"
    )]
    Const(Ident, Type, Option<Expr>),
}

/// TODO: decide whether to allow where predicates
#[derive(Clone, Debug, PartialEq, Display)]
pub enum WherePredicate {
    #[display(fmt = "{}: {}", _0, _1)]
    Type(Type, Add<Path>),
    #[display(fmt = "const {} = {}", _0, _1)]
    Const(Ident, Expr),
}

#[derive(Clone, Debug, PartialEq, Display)]
#[display(
    fmt = "{}",
    "if params.0.is_empty() { \"\".to_string() } else { format!(\"<{}>\", params) }",
    // "where_clause.as_ref().map(|x| format!(\"where: {}\", x)).unwrap_or_default()"
)]
pub struct Generics {
    params: Comma<GenericParam>,
    // where_clause: Option<Comma<WherePredicate>>,
}

#[derive(Clone, Debug, PartialEq, Display)]
#[display(fmt = "{}: {}", ident, ty)]
pub struct Field {
    ident: Ident,
    ty: Type,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Fields {
    #[display(fmt = "{{ {} }}", _0)]
    Named(Comma<Field>),
    #[display(fmt = "({})", _0)]
    Unnamed(Comma<Type>),
    Unit,
}

#[derive(Clone, Debug, PartialEq, Display)]
#[display(
    fmt = "{}{}{}",
    ident,
    fields,
    "discriminant.as_ref().map(|x| format!(\"= {}\", x)).unwrap_or_default()"
)]
pub struct Variant {
    ident: Ident,
    fields: Fields,
    discriminant: Option<Expr>,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum FnArg {
    #[display(fmt = "self")]
    Receiver,
    #[display(fmt = "{}", _0)]
    Typed(PatType),
}

#[derive(Clone, Debug, PartialEq, Display)]
#[display(
    fmt = "{}{}({}){}",
    ident,
    "generics.as_ref().map(|x| format!(\"<{}>\", x)).unwrap_or_default()",
    inputs,
    "output.as_ref().map(|x| format!(\" -> {}\", x)).unwrap_or_default()"
)]
pub struct Sig {
    ident: Ident,
    generics: Option<Generics>,
    inputs: Comma<FnArg>,
    output: Option<Type>,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum ImplItem {
    #[display(fmt = "{}const {}: {} = {}", _0, _1, _2, _3)]
    Const(Vis, Ident, Type, Expr),
    #[display(fmt = "{}fn {}{{ {} }}", _0, _1, _2)]
    Method(Vis, Sig, Block),
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Item {
    /// const x: u3 = 0b000;
    #[display(fmt = "{}const {}: {} = {}", _0, _1, _2, _3)]
    Const(Vis, Ident, Type, Expr),
    #[display(
        fmt = "{}mod {}{}",
        _0,
        _1,
        "_2.as_ref().map(|x| format!(\" {{ {} }}\", x)).unwrap_or(\";\".to_string())"
    )]
    Mod(Vis, Ident, Option<Implicit<Item>>),
    #[display(fmt = "{}fn {}{{ {} }}", _0, _1, _2)]
    Fn(Vis, Sig, Block),
    #[display(fmt = "{}type {}{} = {}", _0, _1, _2, _3)]
    Type(Vis, Ident, Generics, Type),
    #[display(fmt = "{}struct {}{}{}", _0, _1, _2, _3)]
    Struct(Vis, Ident, Generics, Fields),
    /// A special type useful for keeping track of state,
    /// sending named commands, etc.
    /// A discriminant is inferred according to the enum size
    /// and the backing register will be as large as the largest variant.
    #[display(fmt = "{}enum {}{}{}", _0, _1, _2, _3)]
    Enum(Vis, Ident, Generics, Comma<Variant>),
    #[display(fmt = "{}struct {}{}{}", _0, _1, _2, _3)]
    Entity(Vis, Ident, Generics, Fields),
    /// impl X {
    /// }
    #[display(
        fmt = "impl{} {}{} {{ {} }}",
        _0,
        "_1.as_ref().map(|x| format!(\"{} for \", x)).unwrap_or_default()",
        _2,
        _3
    )]
    Impl(Generics, Option<Path>, Box<Type>, Implicit<ImplItem>),

    /// Collection of behaviors that a type satisfies
    /// Entities cannot implement traits, but types can
    Trait,
    /// Import entities, functions, etc. from other modules
    Use,
    /// Don't repeat yourself
    Macro,
}
