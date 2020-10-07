//! Why is this called macaroni?
//! Macro is a reserved identifier and I'm not sure if the lalrpop grammar supports raw identifiers.

use derive_more::Display;

use std::str::FromStr;

use super::*;

macro_rules! macro_enum {
    ($name: ident { $($varname: ident => $varval: expr),+ }) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
        pub enum $name {
            $(
                #[display(fmt = $varval)]
                $varname,
            )+
        }

        impl $name {
            pub fn variants() -> Vec<Self> {
                vec![
                    $(
                        Self::$varname,
                    )+
                ]
            }
        }

        impl FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $varval => Ok(Self::$varname),
                    )+
                    _ => Err(()),
                }
            }
        }
    };
}

macro_enum!(MacroFragSpec {
    Block => "block",
    Expr => "expr",
    Ident => "ident",
    Item => "item",
    Literal => "literal",
    Meta => "block",
    Pat => "block",
    Path => "block",
    Stmt => "block",
    TokenTree =>"tt",
    Type => "ty",
    Visibility => "vis"
});

macro_enum!(MacroRepOp {
    Optional => "?",
    OneOrMore => "+",
    ZeroOrMore => "*"
});

#[derive(Clone, Debug, PartialEq, Display)]
pub enum MacroMatch {
    #[display(fmt = "{}", _0)]
    Token(String),
    #[display(fmt = "{}", _0)]
    Matcher(MacroMatcher),
    #[display(fmt = "${}: {}", _0, _1)]
    Fragment(Ident, MacroFragSpec),
    #[display(fmt = "$({}){}{}", _0, "_1.as_ref().cloned().unwrap_or_default()", _2)]
    Rep(Implicit<Self>, Option<String>, MacroRepOp),
}

#[derive(Clone, Debug, PartialEq, Display)]
pub struct MacroMatcher(pub Implicit<MacroMatch>);

#[derive(Clone, Debug, PartialEq, Display)]
#[display(fmt = "{} => {}", _0, "_1.iter().cloned().collect::<String>()")]
pub struct MacroRule(pub MacroMatcher, pub Vec<String>);
