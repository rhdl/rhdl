use super::{expr::ExprRange, *};

crate::class_from_tokens! {
    Item {
        Mod {
            vis: Option<Vis>,
            mod_token: Mod,
            ident: Ident,
            content: ModContent
        },
        Use {
            vis: Option<Vis>,
            use_token: Use,
            tree: UseTree,
            semi: Semi
        },
        Const {
            vis: Option<Vis>,
            const_token: Const,
            ident: Ident,
            colon: Colon,
            ty: Type,
            eq: Eq,
            expr: Expr,
            semi: Semi
        },
        Fn {
            vis: Option<Vis>,
            fn_token: Fn,
            sig: Sig,
            block: Block
        },
        Type {
            vis: Option<Vis>,
            type_token: super::token::Type,
            ident: Ident,
            generics: Option<Generics>,
            eq: Eq,
            ty: Type,
            semi: Semi
        },
        Struct {
            vis: Option<Vis>,
            struct_token: Struct,
            ident: Ident,
            generics: Option<Generics>,
            fields: Fields,
        },
        Enum {
            vis: Option<Vis>,
            enum_token: Enum,
            ident: Ident,
            generics: Option<Generics>,
            brace_open: BraceOpen,
            variants: Punctuated<Variant, Comma>,
            brace_close: BraceClose
        },
        Impl {
            impl_token: Impl,
            generics: Option<Generics>,
            of: Option<(TypePath, For)>,
            ty: Box<Type>,
            brace_open: BraceOpen,
            items: Vec<ImplItem>,
            brace_close: BraceClose
        },

        // Macro {
        //     vis: Option<Vis>,
        //     macro_token: Macro,
        //     not: Not,

        // }
        Entity {
            vis: Option<Vis>,
            entity: Entity,
            ident: Ident,
            generics: Generics,
            ports: Ports,
            semi: Option<Semi>
        },
        // Bag {
        //     vis: Option<Vis>,
        //     bag: Bag,
        //     ident: Ident,
        //     brace_open: BraceOpen,
        //     literals: Punctuated<Lit, Comma>,
        //     brace_close: BraceClose
        // },
        // Ring {
        //     vis: Option<Vis>,
        //     ring: Ring,
        //     ident: Ident,
        //     eq: Eq,
        //     range: ExprRange,
        //     semi: Semi
        // },
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
            as_token: As,
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
            pub_token: Pub,
            paren_open: ParenOpen,
            crate_token: Crate,
            paren_close: ParenClose
        },
        Super {
            pub_token: Pub,
            paren_open: ParenOpen,
            super_token: Super,
            paren_close: ParenClose
        },
        ExplicitInherited {
            pub_token: Pub,
            paren_open: ParenOpen,
            self_token: LowerSelf,
            paren_close: ParenClose
        },
        Restricted {
            pub_token: Pub,
            paren_open: ParenOpen,
            in_token: In,
            path: SimplePath,
            paren_close: ParenClose
        }
    }
}

crate::insts_from_tokens! {
    Sig {
        ident: Ident,
        generics: Option<Generics>,
        paren_open: ParenOpen,
        inputs: Punctuated<FnArg, Comma>,
        paren_close: ParenClose,
        output: Option<(RArrow, Type)>
    }
}

crate::class_from_tokens! {
    FnArg {
        Receiver {
            inner: LowerSelf
        },
        Typed {
            inner: PatType
        }
    }
}

crate::insts_from_tokens! {
    Generics {
        lt: Lt,
        params: Punctuated<GenericParam, Comma>,
        gt: Gt
    },
    WherePredicate {
        where_token: Where,
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
            const_token: Const,
            ident: Ident,
            colon: Colon,
            type_token: Type,
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
            paren_close: ParenClose,
            semi: Option<Semi>
        }
    }
}

crate::insts_from_tokens! {
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
        Unit {
            inner: Ident
        },
        Discrim {
            ident: Ident,
            eq: Eq,
            expr: Expr
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

crate::class_only_from_tokens! {
    PortType {
        In,
        Out,
        InOut
    }
}

crate::inst_from_tokens! {
    Port {
        port_type: PortType,
        ident: Ident,
        colon: Colon,
        ty: Type
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
