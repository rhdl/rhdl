use super::*;

crate::class_from_tokens! {
    Item {
        Const {
            vis: Option<Vis>,
            ident: Ident,
            colon: Colon,
            ty: Type,
            eq: Eq,
            expr: Expr,
            semi: Semi
        },
        Mod {
            vis: Option<Vis>,
            ident: Ident,
            content: ModContent
        },
        Fn {
            vis: Option<Vis>,
            sig: Sig,
            block: Block
        },
        Type {
            vis: Option<Vis>,
            r#type: super::token::Type,
            ident: Ident,
            generics: Generics,
            eq: Eq,
            ty: Type,
            semi: Semi
        },
        Struct {
            vis: Option<Vis>,
            r#struct: Struct,
            ident: Ident,
            generics: Generics,
            fields: Fields,
            semi: Option<Semi>
        },
        Enum {
            vis: Option<Vis>,
            r#enum: Enum,
            ident: Ident,
            generics: Generics,
            brace_open: BraceOpen,
            variants: Punctuated<Variant, Comma>,
            brace_close: BraceClose
        },
        Impl {
            r#impl: Impl,
            generics: Generics,
            of: Option<(TypePath, For)>,
            ty: Box<Type>,
            brace_open: BraceOpen,
            items: Vec<ImplItem>,
            brace_close: BraceClose
        },
        Use {
            vis: Option<Vis>,
            r#use: Use,
            tree: UseTree,
            semi: Semi
        },
        // Macro {
        //     vis: Option<Vis>,
        //     r#macro: Macro,
        //     not: Not,

        // }
        Entity {
            vis: Option<Vis>,
            ident: Ident,
            generics: Generics,
            fields: Fields,
            semi: Option<Semi>
        }
    }
}

crate::class_from_tokens! {
    ModContent {
        File {
            semi: Semi
        },
        Here {
            brace_open: BraceOpen,
            items: Vec<Item>,
            brace_close: BraceClose
        }
    }
}

crate::class_from_tokens! {
    Vis {
        Pub {
            inner: Pub
        },
        Crate {
            r#pub: Pub,
            paren_open: ParenOpen,
            crate_token: Crate,
            paren_close: ParenClose
        },
        Super {
            r#pub: Pub,
            paren_open: ParenOpen,
            super_token: Super,
            paren_close: ParenClose
        },
        ExplicitInherited {
            r#pub: Pub,
            paren_open: ParenOpen,
            self_token: UpperSelf,
            paren_close: ParenClose
        },
        Restricted {
            r#pub: Pub,
            paren_open: ParenOpen,
            r#in: In,
            path: SimplePath,
            paren_close: ParenClose
        }
    }
}

// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum Vis {
//     #[display(fmt = "pub ")]
//     Pub,
//     #[display(fmt = "pub(crate) ")]
//     Crate,
//     #[display(fmt = "pub(super) ")]
//     Super,
//     #[display(fmt = "pub(self) ")]
//     ExplicitInherited,
//     #[display(fmt = "pub(in {}) ", _0)]
//     Restricted(SimplePath),
//     #[display(fmt = "")]
//     Inherited,
// }

// impl Default for Vis {
//     fn default() -> Self {
//         Self::Inherited
//     }
// }

crate::inst_from_tokens! {
    Sig {
        ident: Ident,
        generics: Generics,
        paren_open: ParenOpen,
        inputs: Punctuated<FnArg, Comma>,
        paren_close: ParenClose,
        output: Option<(RArrow, Type)>
    }
}

crate::inst_from_tokens! {
    Generics {
        lt: Lt,
        params: Punctuated<GenericParam, Comma>,
        where_predicate: Option<WherePredicate>,
        gt: Gt
    },
    WherePredicate {
        r#where: Where,
        params: Punctuated<GenericParam, Comma>
    }
}

crate::class_from_tokens! {
    GenericParam {
        Type {
            ident: Ident,
            bounds: Option<(Colon, Punctuated<TypePath, Plus>)>,
            eq: Option<(Eq, Type)>
        },
        Const {
            r#const: Const,
            ident: Ident,
            colon: Colon,
            r#type: Type,
            default: Option<(Eq, Expr)>
        }
    }
}

// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum GenericParam {
//     #[display(
//         fmt = "{}: {}{}",
//         _0,
//         _1,
//         "_2.as_ref().map(|x| format!(\" = {}\", x)).unwrap_or_default()"
//     )]
//     Type(Ident, Add<TypePath>, Option<Type>),
//     #[display(
//         fmt = "const {}: {}{}",
//         _0,
//         _1,
//         "_2.as_ref().map(|x| format!(\" = {}\", x)).unwrap_or_default()"
//     )]
//     Const(Ident, Type, Option<Expr>),
// }

// TODO: decide whether to allow where predicates
// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum WherePredicate {
//     #[display(fmt = "{}: {}", _0, _1)]
//     Type(Type, Add<TypePath>),
//     #[display(fmt = "const {} = {}", _0, _1)]
//     Const(Ident, Expr),
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// #[display(
//     fmt = "{}",
//     "if params.0.is_empty() { \"\".to_string() } else { format!(\"<{}>\", params) }",
//     // "where_clause.as_ref().map(|x| format!(\"where: {}\", x)).unwrap_or_default()"
// )]
// pub struct Generics {
//     pub params: Comma<GenericParam>,
//     // where_clause: Option<Comma<WherePredicate>>,
// }

crate::class_from_tokens! {
    Fields {
        Named {
            brace_open: BraceOpen,
            inner: Punctuated<NamedField, Comma>,
            brace_close: BraceClose
        },
        Unnamed {
            paren_open: ParenOpen,
            inner: Punctuated<UnnamedField, Comma>,
            paren_close: ParenClose
        }
    }
}

crate::inst_from_tokens! {
    NamedField {
        vis: Option<Vis>,
        ident: Ident,
        colon: Colon,
        ty: Type
    },
    UnnamedField {
        vis: Option<Vis>,
        ty: Type
    }
}

// #[derive(Clone, Debug, PartialEq, Display)]
// #[display(fmt = "{}{}: {}", vis, ident, ty)]
// pub struct NamedField {
//     pub vis: Vis,
//     pub ident: Ident,
//     pub ty: Type,
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// #[display(fmt = "{}{}", vis, ty)]
// pub struct UnnamedField {
//     pub vis: Vis,
//     pub ty: Type,
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum Fields {
//     #[display(fmt = "{{ {} }}", _0)]
//     Named(Comma<NamedField>),
//     #[display(fmt = "({})", _0)]
//     Unnamed(Comma<UnnamedField>),
//     #[display(fmt = "")]
//     Unit,
// }

crate::class_from_tokens! {
    Variant {
        Field {
            ident: Ident,
            fields: Fields
        },
        Discrim {
            ident: Ident,
            eq: Eq,
            expr: Expr
        }
    }
}

crate::class_from_tokens! {
    FnArg {
        Receiver {
            inner: LowerSelf
        },
        Typed {
            pat: PatType
        }
    }
}

// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum Variant {
//     #[display(fmt = "{}{}", _0, _1)]
//     Field(Ident, Fields),
//     #[display(fmt = "{} = {}", _0, _1)]
//     Discrim(Ident, Expr),
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum FnArg {
//     #[display(fmt = "self")]
//     Receiver,
//     #[display(fmt = "{}", _0)]
//     Typed(PatType),
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// #[display(
//     fmt = "{}{}({}){}",
//     ident,
//     generics,
//     inputs,
//     "output.as_ref().map(|x| format!(\" -> {}\", x)).unwrap_or_default()"
// )]
// pub struct Sig {
//     pub ident: Ident,
//     pub generics: Generics,
//     pub inputs: Comma<FnArg>,
//     pub output: Option<Type>,
// }

crate::class_from_tokens! {
    ImplItem {
        Const {
            vis: Option<Vis>,
            r#const: Const,
            ident: Ident,
            colon: Colon,
            ty: Type,
            eq: Eq,
            expr: Expr
        },
        Method {
            vis: Option<Vis>,
            sig: Sig,
            block: Block
        }
    }
}

crate::class_from_tokens! {
    UseTree {
        Path {
            path: SimplePath,
            tree: Box<UseTree>
        },
        Name {
            inner: Ident
        },
        Rename {
            name: Ident,
            r#as: As,
            rename: Ident
        },
        Glob {
            inner: Star
        },
        Group {
            brace_open: BraceOpen,
            trees: Punctuated<UseTree, Comma>,
            brace_close: BraceClose
        }
    }
}

// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum ImplItem {
//     #[display(fmt = "{}const {}: {} = {}", _0, _1, _2, _3)]
//     Const(Vis, Ident, Type, Expr),
//     #[display(fmt = "{}fn {}{{ {} }}", _0, _1, _2)]
//     Method(Vis, Sig, Block),
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum UseTree {
//     #[display(fmt = "{}::{}", _0, _1)]
//     Path(SimplePath, Box<Self>),
//     #[display(fmt = "{}", _0)]
//     Name(Ident),
//     #[display(fmt = "{} as {}", _0, _1)]
//     Rename(Ident, Ident),
//     #[display(fmt = "*")]
//     Glob,
//     #[display(fmt = "{{ {} }}", _0)]
//     Group(Comma<UseTree>),
// }

// #[derive(Clone, Debug, PartialEq, Display)]
// pub enum Item {
//     /// const x: u3 = 0b000;
//     #[display(fmt = "{}const {}: {} = {};", _0, _1, _2, _3)]
//     Const(Vis, Ident, Type, Expr),
//     #[display(
//         fmt = "{}mod {}{}",
//         _0,
//         _1,
//         "_2.as_ref().map(|x| format!(\" {{ {}{}}}\", x, if x.0.is_empty() { \"\" } else { \" \" })).unwrap_or(\";\".to_string())"
//     )]
//     Mod(Vis, Ident, Option<Newline<Item>>),
//     #[display(fmt = "{}fn {} {}", _0, _1, _2)]
//     Fn(Vis, Sig, Block),
//     #[display(fmt = "{}type {}{} = {};", _0, _1, _2, _3)]
//     Type(Vis, Ident, Generics, Type),
//     #[display(
//         fmt = "{}struct {}{}{}{}{}",
//         _0,
//         _1,
//         _2,
//         "if let Fields::Named(_) = _3 { \" \" } else { \"\" }",
//         _3,
//         "if let Fields::Unnamed(_) = _3 { \";\" } else { \"\" }"
//     )]
//     Struct(Vis, Ident, Generics, Fields),
//     #[display(fmt = "{}entity {}{} {}", _0, _1, _2, _3)]
//     Entity(Vis, Ident, Generics, Fields),
//     /// A special type useful for keeping track of state,
//     /// sending named commands, etc.
//     /// A discriminant is inferred according to the enum size
//     /// and the backing register will be as large as the largest variant.
//     /// If all variants are unit variants, an explicit discriminant can be specified.
//     #[display(fmt = "{}enum {}{} {{ {} }}", _0, _1, _2, _3)]
//     Enum(Vis, Ident, Generics, Comma<Variant>),
//     /// impl X {
//     /// }
//     #[display(
//         fmt = "impl{} {}{} {{ {} }}",
//         _0,
//         "_1.as_ref().map(|x| format!(\"{} for \", x)).unwrap_or_default()",
//         _2,
//         _3
//     )]
//     Impl(Generics, Option<TypePath>, Box<Type>, Newline<ImplItem>),
//     /// A type representing a "bag" of allowed literal values for compile-time parameters
//     #[display(fmt = "{}bag {} {{ {} }}", _0, _1, _2)]
//     Bag(Vis, Ident, Comma<Lit>),
//     /// A bounded type closed under addition and subtraction.
//     /// Only other allowed operation is comparison.
//     /// Can be cast to a primitive type where it will have the appropriate bitwise representation.
//     /// It is not possible to cast a primitive into a ring.
//     #[display(
//         fmt = "{}ring {} = {}..{}{};",
//         _0,
//         _1,
//         _2,
//         "if *_4 { \"=\" } else { \"\" }",
//         _3
//     )]
//     Ring(Vis, Ident, Lit, Lit, bool),

//     /// Collection of behaviors that a type satisfies
//     /// Entities cannot implement traits, but types can
//     /// todo: implement traits in ast and grammar
//     Trait,
//     /// Import entities, functions, etc. from other modules
//     #[display(fmt = "{}use {};", _0, _1)]
//     Use(Vis, UseTree),
//     /// Don't repeat yourself
//     #[display(fmt = "{}macro! {} {{ {} }}", _0, _1, _2)]
//     Macro(Vis, Ident, Semi<MacroRule>),
//     #[display(fmt = "{}", _0)]
//     MacroInvocation(MacroInvocation),
// }
