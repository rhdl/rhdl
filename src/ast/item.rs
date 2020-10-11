use derive_more::Display;

use std::fmt::Debug;

use super::*;

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Vis {
    #[display(fmt = "pub ")]
    Pub,
    #[display(fmt = "pub(crate) ")]
    Crate,
    #[display(fmt = "pub(super) ")]
    Super,
    #[display(fmt = "pub(self) ")]
    ExplicitInherited,
    #[display(fmt = "pub({}) ", _0)]
    Restricted(SimplePath),
    #[display(fmt = "")]
    Inherited,
}

impl Default for Vis {
    fn default() -> Self {
        Self::Inherited
    }
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum GenericParam {
    #[display(
        fmt = "{}: {}{}",
        _0,
        _1,
        "_2.as_ref().map(|x| format!(\" = {}\", x)).unwrap_or_default()"
    )]
    Type(Ident, Add<TypePath>, Option<Type>),
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
    Type(Type, Add<TypePath>),
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
    pub params: Comma<GenericParam>,
    // where_clause: Option<Comma<WherePredicate>>,
}

impl Default for Generics {
    fn default() -> Self {
        Self {
            params: Comma(vec![]),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Display)]
#[display(fmt = "{}{}: {}", vis, ident, ty)]
pub struct NamedField {
    pub vis: Vis,
    pub ident: Ident,
    pub ty: Type,
}

#[derive(Clone, Debug, PartialEq, Display)]
#[display(fmt = "{}{}", vis, ty)]
pub struct UnnamedField {
    pub vis: Vis,
    pub ty: Type,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Fields {
    #[display(fmt = "{{ {} }}", _0)]
    Named(Comma<NamedField>),
    #[display(fmt = "({})", _0)]
    Unnamed(Comma<UnnamedField>),
    #[display(fmt = "")]
    Unit,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Variant {
    #[display(fmt = "{}{}", _0, _1)]
    Field(Ident, Fields),
    #[display(fmt = "{} = {}", _0, _1)]
    Discrim(Ident, Expr),
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
    generics,
    inputs,
    "output.as_ref().map(|x| format!(\" -> {}\", x)).unwrap_or_default()"
)]
pub struct Sig {
    pub ident: Ident,
    pub generics: Generics,
    pub inputs: Comma<FnArg>,
    pub output: Option<Type>,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum ImplItem {
    #[display(fmt = "{}const {}: {} = {}", _0, _1, _2, _3)]
    Const(Vis, Ident, Type, Expr),
    #[display(fmt = "{}fn {}{{ {} }}", _0, _1, _2)]
    Method(Vis, Sig, Block),
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum UseTree {
    #[display(fmt = "{}::{}", _0, _1)]
    Path(SimplePath, Box<Self>),
    #[display(fmt = "{}", _0)]
    Name(Ident),
    #[display(fmt = "{} as {}", _0, _1)]
    Rename(Ident, Ident),
    #[display(fmt = "*")]
    Glob,
    #[display(fmt = "{{ {} }}", _0)]
    Group(Comma<UseTree>),
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Item {
    /// const x: u3 = 0b000;
    #[display(fmt = "{}const {}: {} = {};", _0, _1, _2, _3)]
    Const(Vis, Ident, Type, Expr),
    #[display(
        fmt = "{}mod {}{}",
        _0,
        _1,
        "_2.as_ref().map(|x| format!(\" {{ {}{}}}\", x, if x.0.is_empty() { \"\" } else { \" \" })).unwrap_or(\";\".to_string())"
    )]
    Mod(Vis, Ident, Option<Newline<Item>>),
    #[display(fmt = "{}fn {} {}", _0, _1, _2)]
    Fn(Vis, Sig, Block),
    #[display(fmt = "{}type {}{} = {};", _0, _1, _2, _3)]
    Type(Vis, Ident, Generics, Type),
    #[display(
        fmt = "{}struct {}{}{}{}{}",
        _0,
        _1,
        _2,
        "if let Fields::Named(_) = _3 { \" \" } else { \"\" }",
        _3,
        "if let Fields::Unnamed(_) = _3 { \";\" } else { \"\" }"
    )]
    Struct(Vis, Ident, Generics, Fields),
    #[display(fmt = "{}entity {}{} {}", _0, _1, _2, _3)]
    Entity(Vis, Ident, Generics, Fields),
    /// A special type useful for keeping track of state,
    /// sending named commands, etc.
    /// A discriminant is inferred according to the enum size
    /// and the backing register will be as large as the largest variant.
    /// If all variants are unit variants, an explicit discriminant can be specified.
    #[display(fmt = "{}enum {}{} {{ {} }}", _0, _1, _2, _3)]
    Enum(Vis, Ident, Generics, Comma<Variant>),
    /// impl X {
    /// }
    #[display(
        fmt = "impl{} {}{} {{ {} }}",
        _0,
        "_1.as_ref().map(|x| format!(\"{} for \", x)).unwrap_or_default()",
        _2,
        _3
    )]
    Impl(Generics, Option<TypePath>, Box<Type>, Newline<ImplItem>),
    /// A type representing a "bag" of allowed literal values for compile-time parameters
    #[display(fmt = "{}bag {} {{ {} }}", _0, _1, _2)]
    Bag(Vis, Ident, Comma<Lit>),
    /// A bounded type closed under addition and subtraction.
    /// Only other allowed operation is comparison.
    /// Can be cast to a primitive type where it will have the appropriate bitwise representation.
    /// It is not possible to cast a primitive into a ring.
    #[display(
        fmt = "{}ring {} = {}..{}{};",
        _0,
        _1,
        _2,
        "if *_4 { \"=\" } else { \"\" }",
        _3
    )]
    Ring(Vis, Ident, Lit, Lit, bool),

    /// Collection of behaviors that a type satisfies
    /// Entities cannot implement traits, but types can
    /// todo: implement traits in ast and grammar
    Trait,
    /// Import entities, functions, etc. from other modules
    #[display(fmt = "{}use {};", _0, _1)]
    Use(Vis, UseTree),
    /// Don't repeat yourself
    #[display(fmt = "{}macro! {} {{ {} }}", _0, _1, _2)]
    Macro(Vis, Ident, Semi<MacroRule>),
    #[display(fmt = "{}", _0)]
    MacroInvocation(MacroInvocation),
}
