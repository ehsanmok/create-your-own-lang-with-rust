use derive_new::new;
use std::fmt::{self, Display};

use anyhow::bail;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Scalar(ScalarKind),
    Function(Vec<Type>, Box<Type>),
    Unit,
    Unknown,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Type::Scalar(s) => s.fmt(f),
            Type::Function(params, ret) => write!(
                f,
                "({}) -> {}",
                params
                    .iter()
                    .map(|p| format!("{}", p))
                    .collect::<Vec<String>>()
                    .join(", "),
                ret,
            ),
            Type::Unit => write!(f, "unit"),
            Type::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScalarKind {
    Int,
    Bool,
}

impl ScalarKind {
    pub fn is_numeric(&self) -> bool {
        match self {
            ScalarKind::Int => true,
            _ => false,
        }
    }
}

impl fmt::Display for ScalarKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use self::ScalarKind::*;
        let text = match *self {
            Bool => "bool",
            Int => "int",
        };
        f.write_str(text)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, new, Hash)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: Type,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({}): {}", self.kind, self.ty)
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
    Block(Block),
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
            ExprKind::Block(e) => e.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub exprs: Vec<Expr>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{{ {} }}",
            self.exprs
                .iter()
                .map(|p| format!("{}", p))
                .collect::<Vec<String>>()
                .join(", "),
        )
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
    pub ty: Type,
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.ident, self.ty)
    }
}

impl Expr {
    pub fn to_identifier(self) -> Result<Identifier, anyhow::Error> {
        let Expr { kind, ty } = self;
        if let ExprKind::Identifier(ident) = kind {
            Ok(ident)
        } else {
            bail!("Cannot transform Expr to Identifier")
        }
    }
    pub fn to_parameter(self) -> Result<Parameter, anyhow::Error> {
        let Expr { kind, ty } = self;
        if let ExprKind::Identifier(ident) = kind {
            Ok(Parameter::new(ident, ty))
        } else {
            bail!("Cannot transform Expr to Parameter")
        }
    }

    pub fn children(&self) -> Vec<&Expr> {
        match &self.kind {
            ExprKind::Identifier(_) | ExprKind::Literal(_) => vec![],
            ExprKind::UnaryExpr(UnaryExpr { child, .. }) => vec![&**child],
            ExprKind::BinaryExpr(BinaryExpr { lhs, rhs, .. }) => vec![&**lhs, &**rhs],
            ExprKind::Return(Return { value }) => vec![&**value],
            ExprKind::Assignment(Assignment { value, .. }) => vec![&**value],
            ExprKind::Conditional(Conditional {
                cond,
                on_true,
                on_false,
            }) => {
                let mut children = vec![];
                children.append(&mut cond.children());
                children.append(&mut on_true.children());
                children.append(&mut on_false.children());
                children
            }
            ExprKind::Loop(Loop { cond, body }) => {
                let mut children = vec![&**cond];
                children.extend(body.iter());
                children
            }
            ExprKind::Function(Function { body, .. }) => body.iter().collect(),
            ExprKind::Call(Call { args, .. }) => args.iter().collect(),
            ExprKind::Block(Block { exprs }) => exprs.iter().collect(),
        }
    }

    pub fn childern_mut(&mut self) -> Vec<&mut Expr> {
        match &mut self.kind {
            ExprKind::Identifier(_) | ExprKind::Literal(_) => vec![],
            ExprKind::UnaryExpr(UnaryExpr { child, .. }) => vec![&mut **child],
            ExprKind::BinaryExpr(BinaryExpr { lhs, rhs, .. }) => vec![&mut **lhs, &mut **rhs],
            ExprKind::Return(Return { value }) => vec![&mut **value],
            ExprKind::Assignment(Assignment { value, .. }) => vec![&mut **value],
            ExprKind::Conditional(Conditional {
                cond,
                on_true,
                on_false,
            }) => {
                let mut children = vec![];
                children.append(&mut cond.childern_mut());
                children.append(&mut on_true.childern_mut());
                children.append(&mut on_false.childern_mut());
                children
            }
            ExprKind::Loop(Loop { cond, body }) => {
                let mut children = vec![&mut **cond];
                children.extend(body.iter_mut());
                children
            }
            ExprKind::Function(Function { body, .. }) => body.iter_mut().collect(),
            ExprKind::Call(Call { args, .. }) => args.iter_mut().collect(),
            ExprKind::Block(Block { exprs }) => exprs.iter_mut().collect(),
        }
    }

    pub fn traverse(&self, f: &mut impl FnMut(&Expr)) {
        f(self);
        for child in self.children() {
            child.traverse(f);
        }
    }

    pub fn traverse_mut(&mut self, f: &mut impl FnMut(&mut Expr)) {
        f(self);
        for child in self.childern_mut() {
            child.traverse_mut(f);
        }
    }

    pub fn partially_typed(&self) -> bool {
        match &self.ty {
            Type::Unknown => true,
            _ => self.children().iter().any(|c| c.partially_typed()),
        }
    }
}
