use super::{expr::ExprRange, *};

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
        },
        Bag {
            vis: Option<Vis>,
            bag: Bag,
            ident: Ident,
            brace_open: BraceOpen,
            literals: Punctuated<Lit, Comma>,
            brace_close: BraceClose
        },
        Ring {
            vis: Option<Vis>,
            ring: Ring,
            ident: Ident,
            eq: Eq,
            range: ExprRange,
            semi: Semi
        },
        Arch {
            arch: Arch,
            ident: Ident,
            brace_open: BraceOpen,
            items: Vec<ArchItem>,
            brace_close: BraceClose
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

crate::class_from_tokens! {
    ImplItem {
        Const {
            inner: ItemConst
        },
        Method {
            inner: ItemFn
        }
    }
}

crate::class_from_tokens! {
    ArchItem {
        Const {
            inner: ItemConst
        },
        Let {
            inner: StmtLocal
        },
        When {
            when: When,
            expr: Expr,
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
