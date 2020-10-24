use std::boxed::Box;
use std::fmt::Debug;

use paste::paste;

pub mod expr;
pub mod item;
// pub mod macaroni;
pub mod pat;
pub mod token;
pub mod types;

use expr::{Expr, ExprPath};
use item::Item;
use pat::{Pat, PatType};
use token::*;
use types::TypePath;

#[derive(Debug, Clone, PartialEq)]
pub struct Punctuated<T, P>
where
    T: ToTokens + Clone + Debug + PartialEq,
    P: ToTokens + Clone + Debug + PartialEq,
{
    pub inner: Vec<(T, P)>,
    pub last: Option<Box<T>>,
}

impl<T, P> ToTokens for Punctuated<T, P>
where
    T: ToTokens + Clone + Debug + PartialEq,
    P: ToTokens + Clone + Debug + PartialEq,
{
    fn to_tokens(&self) -> Vec<Tok> {
        self.inner
            .iter()
            .map(|(t, p)| {
                let mut t_tokens = t.to_tokens();
                let mut p_tokens = p.to_tokens();
                t_tokens.append(&mut p_tokens);
                t_tokens
            })
            .flatten()
            .chain(
                self.last
                    .as_ref()
                    .map(Box::as_ref)
                    .map(ToTokens::to_tokens)
                    .unwrap_or_default()
                    .drain(..),
            )
            .collect()
    }

    fn first(&self) -> Tok {
        self.inner
            .first()
            .map(|(t, _)| t)
            .map(ToTokens::first)
            .or(self.last.as_ref().map(Box::as_ref).map(ToTokens::first))
            .unwrap()
    }

    fn last(&self) -> Tok {
        self.last
            .as_ref()
            .map(Box::as_ref)
            .map(ToTokens::last)
            .or(self.inner.last().map(|(t, _)| t).map(ToTokens::last))
            .unwrap()
    }

    fn len(&self) -> usize {
        self.inner
            .iter()
            .map(|(t, p)| t.len() + p.len())
            .sum::<usize>()
            + self.last.as_ref().map(|t| t.len()).unwrap_or_default()
    }
}

crate::inst_from_tokens! {
    Block {
        brace_open: BraceOpen,
        statements: Vec<Stmt>,
        brace_close: BraceClose
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimplePath {
    pub leading_sep: Option<PathSep>,
    pub segments: Punctuated<Ident, PathSep>,
}

impl ToTokens for SimplePath {
    fn to_tokens(&self) -> Vec<Tok> {
        let mut acc = self
            .leading_sep
            .as_ref()
            .map(PathSep::to_tokens)
            .unwrap_or_default();
        acc.append(&mut self.segments.to_tokens());
        acc
    }

    fn first(&self) -> Tok {
        self.leading_sep
            .as_ref()
            .map(ToTokens::first)
            .unwrap_or(self.segments.first())
    }

    fn last(&self) -> Tok {
        self.segments.last()
    }

    fn len(&self) -> usize {
        self.leading_sep
            .as_ref()
            .map(ToTokens::len)
            .unwrap_or_default()
            + self.segments.len()
    }
}

#[macro_export]
macro_rules! from_tokens_to_tokens {
    ($field: expr => Option<($a: ty, $b: ty, $c: ty)>) => {
        $field
            .as_ref()
            .map(|(a, b, c)| {
                let mut acc = a.to_tokens();
                acc.append(&mut b.to_tokens());
                acc.append(&mut c.to_tokens());
                acc
            })
            .unwrap_or_default()
    };

    ($field: expr => Option<($a: ty, $b: ty)>) => {
        $field
            .as_ref()
            .map(|(a, b)| {
                let mut acc = a.to_tokens();
                acc.append(&mut b.to_tokens());
                acc
            })
            .unwrap_or_default()
    };

    ($field: expr => Vec<$ty: ty>) => {
        $field
            .iter()
            .map(ToTokens::to_tokens)
            .flatten()
            .collect::<Vec<Tok>>()
    };

    ($field: expr => Box<$ty: ty>) => {
        $field.as_ref().to_tokens()
    };

    ($field: expr => Option<$ty: ty>) => {
        $field.as_ref().map(|x| x.to_tokens()).unwrap_or_default()
    };

    ($field: expr => $ty: ty) => {
        $field.to_tokens()
    };
}

#[macro_export]
macro_rules! inst_from_tokens {
    ($($inst: ident {
        $(
            $member_ident: ident : $member_ty: ty
        ),*
    }),+) => {
        $(
            #[derive(Clone, Debug, PartialEq)]
            pub struct $inst {
                $(
                    pub $member_ident: $member_ty
                ),*
            }

            impl ToTokens for $inst {
                paste! {
                    fn to_tokens(&self) -> Vec<Tok> {
                        $(
                            let mut [<$member_ident tokens>] = crate::from_tokens_to_tokens!(self.$member_ident => $member_ty);
                        )*
                        let mut acc = Vec::with_capacity(0 $( + [<$member_ident tokens>].len() )*);
                        $(
                            acc.append(&mut [<$member_ident tokens>]);
                        )*
                        acc
                    }
                }

                fn first(&self) -> Tok {
                    self.to_tokens().first().cloned().unwrap()
                }

                fn last(&self) -> Tok {
                    self.to_tokens().last().cloned().unwrap()
                }

                fn len(&self) -> usize {
                    self.to_tokens().len()
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! class_only_from_tokens {
    ($class:ident {
        $( $variant: ident ),*
    }) => {
        paste! {
            #[derive(Clone, Debug, PartialEq)]
            pub enum $class {
                $(
                    $variant([<$variant>])
                ),*
            }
        }

        impl ToTokens for $class {
            fn to_tokens(&self) -> Vec<Tok> {
                match self {
                    $( Self::$variant(x) => x.to_tokens() ),*
                }
            }

            fn first(&self) -> Tok {
                match self {
                    $( Self::$variant(x) => x.first() ),*
                }
            }

            fn last(&self) -> Tok {
                match self {
                    $( Self::$variant(x) => x.last() ),*
                }
            }

            fn len(&self) -> usize {
                match self {
                    $( Self::$variant(x) => x.len() ),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! class_from_tokens {
    ($class: ident {
        $($variant: ident {
            $(
                $member_ident: ident : $member_ty: ty
            ),*
        }),*
    }) => {
        paste! {
            $(crate::inst_from_tokens! {
                [<$class $variant>] {
                    $(
                        $member_ident : $member_ty
                    ),*
                }
            })*
            crate::class_only_from_tokens! {
                $class {
                    $( [<$class $variant>] ),*
                }
            }
        }
    };
}

crate::class_from_tokens! {
    Stmt {
        Local {
            r#let: Let,
            pat: Pat,
            ty: Option<(Colon, Type)>,
            init: Option<(Eq, Box<Expr>)>,
            semi: Semi
        },
        Item {
            item: Item
        },
        Expr {
            expr: Expr,
            semi: Option<Semi>
        }
    }
}

crate::class_from_tokens! {
    RangeType {
        HalfOpen {
            inner: DotDot
        },
        Closed {
            inner: DotDotEq
        }
    }
}
