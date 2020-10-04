use derive_more::Display;
use rug::{Float, Integer as Int};
use std::str::FromStr;

use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Comma<T: Clone + Display + Debug + PartialEq>(pub Vec<T>);

impl<T: Clone + Display + Debug + PartialEq> Display for Comma<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(item) = self.0.first() {
            write!(f, "{}", item)?;
        }
        for item in self.0.iter().skip(1) {
            write!(f, ", {}", item)?;
        }
        Ok(())
    }
}

impl<T: Clone + Display + Debug + PartialEq> From<Vec<T>> for Comma<T> {
    fn from(v: Vec<T>) -> Self {
        Self(v)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block(pub Vec<Stmt>);

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for stmt in self.0.iter().take(self.0.len().saturating_sub(1)) {
            write!(f, "{} ", stmt)?;
        }
        if let Some(stmt) = self.0.last() {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    /// const x: u3 = 0b000;
    Const {
        ident: Ident,
        ty: Type,
        expr: Expr,
    },
    /// impl X {
    /// }
    Impl,
    Entity,
    /// A special type useful for keeping track of state,
    /// sending named commands, etc.
    /// A discriminator is inferred according to the enum size
    /// and the backing register will be as large as the largest variant.
    Enum,
    /// Functions can have a self ref
    Fn,
    /// Collection of behaviors that a type satisfies
    /// Entities cannot implement traits, but types can
    Trait,
    /// Import entities, functions, etc. from other modules
    Use,
    /// Don't repeat yourself
    Macro,
    /// entity Sawtooth16Bit = Sawtooth<u16>;
    EntityAlias,
    Mod,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Type {
    #[display(fmt = "[{}; {}]", _0, _1)]
    Array(Box<Type>, Box<Lit>),
    #[display(fmt = "{}", _0)]
    Path(Path),
    #[display(
        fmt = "fn({}){}",
        _0,
        "_1.as_ref().map(|x| format!(\" -> ({})\", x)).unwrap_or_default()"
    )]
    Fn(Comma<Type>, Option<Box<Type>>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
#[display(fmt = "{}", inner)]
pub struct Ident {
    pub inner: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Path {
    pub leading_colon: Option<()>,
    pub segments: Vec<Ident>,
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, seg) in self.segments.iter().enumerate() {
            write!(
                f,
                "{}{}",
                if i != 0 || self.leading_colon.is_some() {
                    "::"
                } else {
                    ""
                },
                seg
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Pat {
    #[display(fmt = "{}", _0)]
    Ident(Ident),
    #[display(fmt = "{}", _0)]
    Lit(Lit),
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Stmt {
    #[display(
        fmt = "(let {}{} {});",
        _0,
        "_1.as_ref().map(|x| format!(\": {}\", x)).unwrap_or_default()",
        "_2.as_ref().map(|x| format!(\"{}\", x)).unwrap_or_default()"
    )]
    Local(Pat, Option<Type>, Option<Expr>),
    // Item(),
    #[display(fmt = "{}{}", _0, "if *_1 { \";\" } else { \"\" }")]
    Expr(Expr, bool),
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Expr {
    // Block(Block),
    #[display(fmt = "({} {})", _0, _1)]
    Unary(UnOp, Box<Expr>),
    #[display(fmt = "({} {} {})", _1, _0, _2)]
    Binary(Box<Expr>, BinOp, Box<Expr>),
    #[display(fmt = "({} {} {})", _1, _0, _2)]
    Assign(Box<Expr>, AssOp, Box<Expr>),
    #[display(fmt = "[{}; {}]", _0, _1)]
    Repeat(Box<Expr>, Box<Expr>),
    /// From, to, closed-ness
    #[display(
        fmt = "{}..{}{}",
        "_0.as_ref().map(|x| format!(\"{}\", x)).unwrap_or_default()",
        "if *_2 { \"=\" } else { \"\" }",
        "_1.as_ref().map(|x| format!(\"{}\", x)).unwrap_or_default()"
    )]
    Range(Option<Box<Expr>>, Option<Box<Expr>>, bool),
    #[display(fmt = "{}", _0)]
    Concat(Comma<Expr>),
    #[display(fmt = "{}", _0)]
    Path(Path),
    #[display(fmt = "{}", _0)]
    Lit(Lit),
    #[display(fmt = "{}.{}", _0, _1)]
    Field(Box<Expr>, Member),
    #[display(fmt = "{}({})", _0, _1)]
    Call(Box<Expr>, Comma<Expr>),
    #[display(fmt = "{}.{}({})", _0, _1, _2)]
    MethodCall(Box<Expr>, Ident, Comma<Expr>),
    #[display(fmt = "{}[{}]", _0, _1)]
    Index(Box<Expr>, Box<Expr>),
    #[display(fmt = "[{}]", _0)]
    Array(Comma<Expr>),
    #[display(fmt = "({})", _0)]
    Tuple(Comma<Expr>),
    #[display(fmt = "(as {} {})", _0, _1)]
    Cast(Box<Expr>, Box<Type>),
    #[display(
        fmt = "(if {} {}{})",
        _0,
        _1,
        "_2.as_ref().map(|x| format!(\" (else {})\", x)).unwrap_or_default()"
    )]
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    #[display(fmt = "(match {} {})", _0, _1)]
    Match(Box<Expr>, Comma<Arm>),
    #[display(fmt = "{}", _0)]
    Block(Block),
    #[display(
        fmt = "return {}",
        "_0.as_ref().map(|x| format!(\"{}\", x)).unwrap_or_default()"
    )]
    Return(Option<Box<Expr>>),
    #[display(fmt = "{}", _0)]
    Struct(Path, Comma<FieldValue>, Option<Box<Expr>>),
}

#[derive(Clone, Debug, PartialEq, Display)]
#[display(
    fmt = "(=> {}{} {})",
    pat,
    "guard.as_ref().map(|x| format!(\"(if {})\", x)).unwrap_or_default()",
    body
)]
pub struct Arm {
    pub pat: Pat,
    pub guard: Option<Expr>,
    pub body: Expr,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
pub enum Member {
    #[display(fmt = "{}", _0)]
    Named(Ident),
    #[display(fmt = "{}", _0)]
    Unnamed(Int),
}

#[derive(Clone, Debug, PartialEq, Display)]
#[display(fmt = "{}: {}", member, expr)]
pub struct FieldValue {
    pub member: Member,
    pub expr: Expr,
}

macro_rules! op_enum {
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

op_enum!(BinOp {
    Add => "+",
    Sub => "-",
    Mul => "*",
    Div => "/",
    Rem => "%",
    Exp => "**",
    // And => "&&",
    // Or => "||",
    BitXor => "^",
    BitAnd => "&",
    BitOr => "|",
    Shl => "<<",
    Shr => ">>",
    Eq => "==",
    Lt => "<",
    Le => "<=",
    Ne => "!=",
    Ge => ">=",
    Gt => ">"
});

op_enum!( UnOp {
    Not => "!",
    Neg => "-"
});

op_enum!(AssOp {
    Eq => "=",
    AddEq => "+=",
    SubEq => "-=",
    MulEq => "*=",
    DivEq => "/=",
    RemEq => "%=",
    ExpEq => "**=",
    BitXorEq => "^=",
    BitAndEq => "&=",
    BitOrEq => "|=",
    ShlEq => "<<=",
    ShrEq => ">>="
});

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Pattern {}

/// let binding
#[derive(Clone, Debug, PartialEq)]
pub struct Let {
    pat: Pattern,
    ty: Option<Type>,
    init: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Lit {
    #[display(fmt = "{}", val)]
    Int {
        val: Int,
        /// i.e. i32
        type_hint: Option<(char, usize)>,
    },
    #[display(fmt = "{}", val)]
    Float { val: Float },
}
