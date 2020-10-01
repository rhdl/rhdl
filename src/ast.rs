use derive_more::Display;
use rug::{Float, Integer as Int};

use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Clone, Debug, PartialEq)]
pub struct Comma<T: Clone + Display + Debug + PartialEq>(pub Vec<T>);

impl<T: Clone + Display + Debug + PartialEq> Display for Comma<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(item) = self.0.first() {
            write!(f, "{}", item)?;
        }
        for item in self.0.iter().skip(1) {
            write!(f, ",{}", item)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block(pub Vec<Stmt>);

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Stmt {
    #[display(
        fmt = "(let {}{} {})",
        _0,
        "_1.as_ref().map(|x| format!(\": {}\", x)).unwrap_or_default()",
        _2
    )]
    Local(Pat, Option<Type>, Expr),
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
    Call(Path, Comma<Expr>),
    #[display(fmt = "{}.{}({})", _0, _1, _2)]
    MethodCall(Box<Expr>, Ident, Comma<Expr>),
    #[display(fmt = "{}[{}]", _0, _1)]
    Index(Box<Expr>, Box<Expr>),
    #[display(fmt = "[{}]", _0)]
    Array(Comma<Expr>),
    #[display(fmt = "(as {} {})", _0, _1)]
    Cast(Box<Expr>, Box<Type>),
    #[display(
        fmt = "(if {} ({}){})",
        _0,
        _1,
        "_2.as_ref().map(|x| format!(\" (else {})\", x)).unwrap_or_default()"
    )]
    If(
        Box<Expr>,
        Block,
        Option<Box<Expr>>,
    ),
    #[display(fmt = "{}", _0)]
    Block(Block),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
pub enum Member {
    #[display(fmt = "{}", _0)]
    Named(Ident),
    #[display(fmt = "{}", _0)]
    Unnamed(Int),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
pub enum BinOp {
    #[display(fmt = "+")]
    Add,
    #[display(fmt = "-")]
    Sub,
    #[display(fmt = "*")]
    Mul,
    #[display(fmt = "/")]
    Div,
    #[display(fmt = "%")]
    Rem,
    #[display(fmt = "&&")]
    And,
    #[display(fmt = "||")]
    Or,
    #[display(fmt = "^")]
    BitXor,
    #[display(fmt = "&")]
    BitAnd,
    #[display(fmt = "|")]
    BitOr,
    #[display(fmt = "<<")]
    Shl,
    #[display(fmt = ">>")]
    Shr,
    #[display(fmt = "==")]
    Eq,
    #[display(fmt = "<")]
    Lt,
    #[display(fmt = "<=")]
    Le,
    #[display(fmt = "!=")]
    Ne,
    #[display(fmt = ">=")]
    Ge,
    #[display(fmt = ">")]
    Gt,
    #[display(fmt = "**")]
    Exp,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
pub enum UnOp {
    #[display(fmt = "!")]
    Not,
    #[display(fmt = "-")]
    Neg,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Display)]
pub enum AssOp {
    #[display(fmt = "=")]
    Eq,
    #[display(fmt = "+=")]
    AddEq,
    #[display(fmt = "-=")]
    SubEq,
    #[display(fmt = "*=")]
    MulEq,
    #[display(fmt = "/=")]
    DivEq,
    #[display(fmt = "%=")]
    RemEq,
    #[display(fmt = "^=")]
    BitXorEq,
    #[display(fmt = "&=")]
    BitAndEq,
    #[display(fmt = "|=")]
    BitOrEq,
    #[display(fmt = "<<=")]
    ShlEq,
    #[display(fmt = ">>=")]
    ShrEq,
}

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
