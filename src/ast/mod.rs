use std::boxed::Box;
use std::fmt::Debug;

use paste::paste;

mod expr;
mod item;
// pub mod macaroni;
mod pat;
pub(crate) mod token;
mod types;

pub use expr::*;
pub use item::*;
pub use pat::*;
use token::*;
pub use token::{Ident, Lit, LitFloat, LitInt, Span, Spanned, ToTokens, Tok};
pub use types::*;

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

    fn first(&self) -> Option<Tok> {
        self.inner
            .first()
            .map(|(t, _)| t)
            .and_then(ToTokens::first)
            .or(self
                .last
                .as_ref()
                .map(Box::as_ref)
                .and_then(ToTokens::first))
    }

    fn last(&self) -> Option<Tok> {
        self.last
            .as_ref()
            .map(Box::as_ref)
            .and_then(ToTokens::last)
            .or(self.inner.last().map(|(t, _)| t).and_then(ToTokens::last))
    }

    fn len(&self) -> usize {
        self.inner
            .iter()
            .map(|(t, p)| t.len() + p.len())
            .sum::<usize>()
            + self.last.as_ref().map(|t| t.len()).unwrap_or_default()
    }
}

impl<'ast, T, P> Punctuated<T, P>
where
    T: ToTokens + Clone + Debug + PartialEq,
    P: ToTokens + Clone + Debug + PartialEq,
{
    pub fn iter(&'ast self) -> impl Iterator<Item = &'ast T> {
        self.inner
            .iter()
            .map(|(t, _)| t)
            .chain(self.last.iter().map(|last| last.as_ref()))
    }
}

crate::insts_from_tokens! {
    Block {
        brace_open: BraceOpen,
        statements: Vec<Stmt>,
        brace_close: BraceClose
    },
    SimplePath {
        leading_sep: Option<PathSep>,
        segments: Punctuated<Ident, PathSep>
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
macro_rules! call_visitors {
    ($v: expr, $field: expr => Option<($a: ty, $b: ty, Punctuated<$d: ty, $e: ty>)>) => {
        if let Some((a, b, c)) = $field.as_ref() {
            crate::call_visitors!($v, a => $a);
            crate::call_visitors!($v, b => $b);
            c.inner.iter().for_each(|(d, e)| {
                crate::call_visitors!($v, d => $d);
                crate::call_visitors!($v, e => $e);
            });
            if let Some(last) = c.last.as_ref() {
                crate::call_visitors!($v, last.as_ref() => $d);
            }
        }
    };

    ($v: expr, $field: expr => Option<($a: ty, $b: ty, $c: ty)>) => {
        if let Some((a, b, c)) = $field.as_ref() {
            crate::call_visitors!($v, a => $a);
            crate::call_visitors!($v, b => $b);
            crate::call_visitors!($v, c => $c);
        }
    };

    ($v: expr, $field: expr => Option<($a: ty, Punctuated<$c: ty, $d: ty>)>) => {
        if let Some((a, b)) = $field.as_ref() {
            crate::call_visitors!($v, a => $a);
            b.inner.iter().for_each(|(c, d)| {
                crate::call_visitors!($v, c => $c);
                crate::call_visitors!($v, d => $d);
            });
            if let Some(last) = b.last.as_ref() {
                crate::call_visitors!($v, last.as_ref() => $c);
            }
        }
    };


    ($v: expr, $field: expr => Option<($a: ty, Box<$b: ty>)>) => {
        if let Some((a, b)) = $field.as_ref() {
            crate::call_visitors!($v, a => $a);
            crate::call_visitors!($v, b.as_ref() => $b);
        }
    };

    ($v: expr, $field: expr => Option<($a: ty, $b: ty)>) => {
        if let Some((a, b)) = $field.as_ref() {
            crate::call_visitors!($v, a => $a);
            crate::call_visitors!($v, b => $b);
        }
    };

    ($v: expr, $field: expr => Vec<$ty: ty>) => {
        for field in $field.iter() {
            crate::call_visitors!($v, field => $ty)
        }
    };

    ($v: expr, $field: expr => Option<Box<$ty: ty>>) => {
        if let Some(field) = $field.as_ref() {
            crate::call_visitors!($v, field.as_ref() => $ty);
        }
    };

    ($v: expr, $field: expr => Option<$ty: ty>) => {
        if let Some(field) = $field.as_ref() {
            crate::call_visitors!($v, field => $ty);
        }
    };

    ($v: expr, $field: expr => Punctuated<$a: ty, $b: ty>) => {
        $field.inner.iter().for_each(|(a, b)| {
            crate::call_visitors!($v, a => $a);
            crate::call_visitors!($v, b => $b);
        });
        if let Some(last) = $field.last.as_ref() {
            crate::call_visitors!($v, last.as_ref() => $a);
        }
    };

    ($v: expr, $field: expr => Box<$ty: ty>) => {
        crate::call_visitors!($v, $field.as_ref() => $ty);
    };

    ($v: expr, $field: expr => $ty: ty) => {
        paste! {
            $v.[<visit_ $ty:snake>](&$field)
        }
    };
}

#[macro_export]
macro_rules! inst_from_tokens {
    ($inst: ident { }) => {
        pub type $inst = ();
        paste! {
            pub(crate) fn [<visit_ $inst:snake>]<'ast, V>(v: &mut V, inst: &'ast $inst) where V: crate::visit::Visit<'ast> + ?Sized { }
        }

        impl ToTokens for $inst {
            fn to_tokens(&self) -> Vec<Tok> { vec![] }

            fn first(&self) -> Option<Tok> { None }

            fn last(&self) -> Option<Tok> { None }

            fn len(&self) -> usize { 0 }
        }
    };
    ($inst: ident {
            inner : $member_ty: ty
    }) => {
        pub type $inst = $member_ty;


        paste! {
            pub(crate) fn [<visit_ $inst:snake>]<'ast, V>(v: &mut V, inst: &'ast $inst) where V: crate::visit::Visit<'ast> + ?Sized {
                crate::call_visitors!(v, inst => $member_ty)
            }
        }
    };
    ($inst: ident {
        $(
            $member_ident: ident : $member_ty: ty
        ),+
    }) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $inst {
            $(
                pub $member_ident: $member_ty
            ),*
        }

        paste! {
            pub(crate) fn [<visit_ $inst:snake>]<'ast, V>(v: &mut V, inst: &'ast $inst) where V: crate::visit::Visit<'ast> + ?Sized {
                $(
                    crate::call_visitors!(v, inst.$member_ident => $member_ty);
                )*
            }
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

            fn first(&self) -> Option<Tok> {
                self.to_tokens().first().cloned()
            }

            fn last(&self) -> Option<Tok> {
                self.to_tokens().last().cloned()
            }

            fn len(&self) -> usize {
                self.to_tokens().len()
            }
        }
    };
}

#[macro_export]
macro_rules! insts_from_tokens {
    ($($inst: ident {
        $(
            $member_ident: ident : $member_ty: ty
        ),*
    }),+) => {
        $(
            crate::inst_from_tokens! {
                $inst {
                    $(
                        $member_ident: $member_ty
                    ),*
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
                    $variant($variant)
                ),*
            }

            pub(crate) fn [<visit_ $class:snake>]<'ast, V>(v: &mut V, inst: &'ast $class) where V: crate::visit::Visit<'ast> + ?Sized {
                match inst {
                    $( $class::$variant(variant) => v.[<visit _ $variant:snake>](variant) ),*
                }
            }
        }

        impl ToTokens for $class {
            fn to_tokens(&self) -> Vec<Tok> {
                match self {
                    $( Self::$variant(x) => x.to_tokens() ),*
                }
            }

            fn first(&self) -> Option<Tok> {
                match self {
                    $( Self::$variant(x) => x.first() ),*
                }
            }

            fn last(&self) -> Option<Tok> {
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

        impl $class {
            pub fn variants() -> Vec<String> {
                vec![
                    $(ToString::to_string(&$variant { left: 0 }) ),*
                ]
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
            $(crate::insts_from_tokens! {
                [<$class $variant>] {
                    $(
                        $member_ident : $member_ty
                    ),*
                }
            })*

            pub(crate) fn [<visit_ $class:snake>]<'ast, V>(v: &mut V, inst: &'ast $class) where V: crate::visit::Visit<'ast> + ?Sized {
                match inst {
                    $( $class::$variant(variant) => v.[<visit_ $class:snake _ $variant:snake>](variant) ),*
                }
            }

            #[derive(Clone, Debug, PartialEq)]
            pub enum $class {
                $(
                    $variant([<$class $variant>])
                ),*
            }
            impl ToTokens for $class {
                fn to_tokens(&self) -> Vec<Tok> {
                    match self {
                        $( Self::$variant(x) => x.to_tokens() ),*
                    }
                }

                fn first(&self) -> Option<Tok> {
                    match self {
                        $( Self::$variant(x) => x.first() ),*
                    }
                }
                fn last(&self) -> Option<Tok> {
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
        }
    };
}

crate::class_from_tokens! {
    Stmt {
        Local {
            let_token: Let,
            pat: Pat,
            ty: Option<(Colon, Type)>,
            init: Option<(Eq, Box<Expr>)>,
            semi: Semi
        },
        Item {
            inner: Item
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
