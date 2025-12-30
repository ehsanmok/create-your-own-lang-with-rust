//! Typed Abstract Syntax Tree for Secondlang
//!
//! Every expression carries its inferred type, enabling LLVM code generation.

use crate::types::Type;
use std::fmt;

/// A program is a list of typed statements
pub type Program = Vec<Stmt>;

// ANCHOR: stmt
/// Statements in our language
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// Function definition with types
    Function {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Stmt>,
    },
    /// Return statement
    Return(TypedExpr),
    /// Assignment with optional type annotation
    Assignment {
        name: String,
        type_ann: Option<Type>,
        value: TypedExpr,
    },
    /// Expression statement
    Expr(TypedExpr),
}
// ANCHOR_END: stmt

// ANCHOR: typed_expr
/// A typed expression: expression + its inferred type
#[derive(Debug, Clone, PartialEq)]
pub struct TypedExpr {
    pub expr: Expr,
    pub ty: Type,
}

impl TypedExpr {
    pub fn new(expr: Expr, ty: Type) -> Self {
        TypedExpr { expr, ty }
    }

    pub fn unknown(expr: Expr) -> Self {
        TypedExpr {
            expr,
            ty: Type::Unknown,
        }
    }
}
// ANCHOR_END: typed_expr

// ANCHOR: expr
/// Expressions in our language
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Integer literal
    Int(i64),
    /// Boolean literal
    Bool(bool),
    /// Variable reference
    Var(String),
    /// Unary operation
    Unary { op: UnaryOp, expr: Box<TypedExpr> },
    /// Binary operation
    Binary {
        op: BinaryOp,
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    /// Function call
    Call { name: String, args: Vec<TypedExpr> },
    /// Conditional
    If {
        cond: Box<TypedExpr>,
        then_branch: Vec<Stmt>,
        else_branch: Vec<Stmt>,
    },
    /// While loop
    While {
        cond: Box<TypedExpr>,
        body: Vec<Stmt>,
    },
    /// Block
    Block(Vec<Stmt>),
}
// ANCHOR_END: expr

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg, // -
    Not, // !
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Comparison
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
}

// Display implementations

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Function {
                name,
                params,
                return_type,
                ..
            } => {
                let params_str: Vec<_> = params
                    .iter()
                    .map(|(n, t)| format!("{}: {}", n, t))
                    .collect();
                write!(
                    f,
                    "def {}({}) -> {} {{ ... }}",
                    name,
                    params_str.join(", "),
                    return_type
                )
            }
            Stmt::Return(expr) => write!(f, "return {}", expr),
            Stmt::Assignment {
                name,
                type_ann,
                value,
            } => {
                if let Some(t) = type_ann {
                    write!(f, "{}: {} = {}", name, t, value)
                } else {
                    write!(f, "{} = {}", name, value)
                }
            }
            Stmt::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

impl fmt::Display for TypedExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.expr, self.ty)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Int(n) => write!(f, "{}", n),
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Var(name) => write!(f, "{}", name),
            Expr::Unary { op, expr } => write!(f, "({}{})", op, expr.expr),
            Expr::Binary { op, left, right } => {
                write!(f, "({} {} {})", left.expr, op, right.expr)
            }
            Expr::Call { name, args } => {
                let args_str: Vec<_> = args.iter().map(|a| a.expr.to_string()).collect();
                write!(f, "{}({})", name, args_str.join(", "))
            }
            Expr::If { cond, .. } => write!(f, "if ({}) {{ ... }} else {{ ... }}", cond.expr),
            Expr::While { cond, .. } => write!(f, "while ({}) {{ ... }}", cond.expr),
            Expr::Block(_) => write!(f, "{{ ... }}"),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Mod => write!(f, "%"),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::Le => write!(f, "<="),
            BinaryOp::Ge => write!(f, ">="),
            BinaryOp::Eq => write!(f, "=="),
            BinaryOp::Ne => write!(f, "!="),
        }
    }
}
