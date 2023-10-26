use derive_new::new;
use std::fmt::{self, Display};

use anyhow::bail;

#[derive(Debug, Clone, PartialEq, Eq, new, Hash)]
pub struct Expr {
    pub kind: ExprKind,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({})", self.kind)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprKind {
    Identifier(Identifier),
    Literal(LiteralKind),
    UnaryExpr(UnaryExpr),
    BinaryExpr(BinaryExpr),
    Return(Return),
    Assignment(Assignment),
    Conditional(Conditional),
    Loop(Loop),
    Function(Function),
    Call(Call),
    Block(Vec<Expr>),
}

impl fmt::Display for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ExprKind::Identifier(ident) => ident.fmt(f),
            ExprKind::Literal(lk) => lk.fmt(f),
            ExprKind::UnaryExpr(e) => e.fmt(f),
            ExprKind::BinaryExpr(e) => e.fmt(f),
            ExprKind::Return(e) => e.fmt(f),
            ExprKind::Assignment(e) => e.fmt(f),
            ExprKind::Conditional(e) => e.fmt(f),
            ExprKind::Loop(e) => e.fmt(f),
            ExprKind::Function(e) => e.fmt(f),
            ExprKind::Call(e) => e.fmt(f),
            ExprKind::Block(e) => write!(
                f,
                "{{ {} }}",
                e.iter()
                    .map(|p| format!("{}", p))
                    .collect::<Vec<String>>()
                    .join(", "),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, new, Hash)]
pub struct Identifier {
    pub name: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    Int(i64),
    Bool(bool),
}

impl fmt::Display for LiteralKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            LiteralKind::Int(n) => write!(f, "{}", n),
            LiteralKind::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, new, Hash)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub child: Box<Expr>,
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}{}", self.op, self.child)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Plus,
    Minus,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            UnaryOp::Plus => write!(f, "+"),
            UnaryOp::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, new, Hash)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    LessThan,
    GreaterThan,
}

impl BinaryOp {
    pub fn is_comparison(&self) -> bool {
        match self {
            BinaryOp::LessThan | BinaryOp::GreaterThan => true,
            _ => false,
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::LessThan => write!(f, "<"),
            BinaryOp::GreaterThan => write!(f, ">"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, new, Hash)]
pub struct Return {
    pub value: Box<Expr>,
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "return {}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, new, Hash)]
pub struct Assignment {
    pub ident: Identifier,
    pub value: Box<Expr>,
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} = {}", self.ident, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, new, Hash)]
pub struct Conditional {
    pub cond: Box<Expr>,
    pub on_true: Box<Expr>,
    pub on_false: Box<Expr>,
}

impl fmt::Display for Conditional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "if ({}) {{ {} }} else {{ {} }}",
            self.cond, self.on_true, self.on_false
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, new, Hash)]
pub struct Loop {
    pub cond: Box<Expr>,
    pub body: Box<Vec<Expr>>,
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "while ({}) {{ {} }}",
            self.cond,
            self.body
                .iter()
                .map(|p| format!("{}", p))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, new, Hash)]
pub struct Call {
    pub ident: Identifier,
    pub args: Vec<Expr>,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}({})",
            self.ident,
            self.args
                .iter()
                .map(|p| format!("{}", p))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, new, Hash)]
pub struct Function {
    pub ident: Identifier,
    pub params: Vec<Parameter>,
    // This only supports single expression bodies
    // pub body: Box<Expr>,
    pub body: Box<Vec<Expr>>, // i.e. a block
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "def {}({}) {{ {} }}",
            self.ident,
            self.params
                .iter()
                .map(|p| format!("{}", p))
                .collect::<Vec<String>>()
                .join(", "),
            self.body
                .iter()
                .map(|p| format!("{}", p))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, new, Hash)]
pub struct Parameter {
    pub ident: Identifier,
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}

impl Expr {
    pub fn to_identifier(self) -> Result<Identifier, anyhow::Error> {
        let Expr { kind } = self;
        if let ExprKind::Identifier(ident) = kind {
            Ok(ident)
        } else {
            bail!("Cannot transform Expr to Identifier")
        }
    }
    pub fn to_parameter(self) -> Result<Parameter, anyhow::Error> {
        let Expr { kind } = self;
        if let ExprKind::Identifier(ident) = kind {
            Ok(Parameter::new(ident))
        } else {
            bail!("Cannot transform Expr to Parameter")
        }
    }
}
