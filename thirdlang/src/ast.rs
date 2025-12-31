//! Typed Abstract Syntax Tree for Thirdlang
//!
//! Extends Secondlang's AST with class definitions, method calls,
//! field access, new/delete expressions.

use crate::types::Type;
use std::fmt;

/// A program is a list of top-level items (classes and statements)
pub type Program = Vec<TopLevel>;

// ANCHOR: top_level
/// Top-level items in a program
#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel {
    /// Class definition
    Class(ClassDef),
    /// Statement (function definition or expression)
    Stmt(Stmt),
}
// ANCHOR_END: top_level

// ANCHOR: class_def
/// Class definition
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDef {
    /// Class name
    pub name: String,
    /// Field definitions (in order)
    pub fields: Vec<FieldDef>,
    /// Method definitions
    pub methods: Vec<MethodDef>,
}

/// Field definition
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDef {
    pub name: String,
    pub ty: Type,
}

/// Method definition
#[derive(Debug, Clone, PartialEq)]
pub struct MethodDef {
    /// Method name (e.g., "__init__", "__del__", "distance")
    pub name: String,
    /// Parameters (excluding self)
    pub params: Vec<(String, Type)>,
    /// Return type
    pub return_type: Type,
    /// Method body
    pub body: Vec<Stmt>,
}

impl MethodDef {
    pub fn is_constructor(&self) -> bool {
        self.name == "__init__"
    }

    pub fn is_destructor(&self) -> bool {
        self.name == "__del__"
    }
}
// ANCHOR_END: class_def

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
        target: AssignTarget,
        type_ann: Option<Type>,
        value: TypedExpr,
    },
    /// Delete statement
    Delete(TypedExpr),
    /// Expression statement
    Expr(TypedExpr),
}
// ANCHOR_END: stmt

// ANCHOR: assign_target
/// Assignment target - can be a variable or field access
#[derive(Debug, Clone, PartialEq)]
pub enum AssignTarget {
    /// Simple variable: x = ...
    Var(String),
    /// Field access: self.x = ... or obj.field = ...
    Field {
        object: Box<TypedExpr>,
        field: String,
    },
}
// ANCHOR_END: assign_target

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
    /// Self reference (inside methods)
    SelfRef,
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
    /// Method call: obj.method(args)
    MethodCall {
        object: Box<TypedExpr>,
        method: String,
        args: Vec<TypedExpr>,
    },
    /// Field access: obj.field
    FieldAccess {
        object: Box<TypedExpr>,
        field: String,
    },
    /// Object creation: new ClassName(args)
    New { class: String, args: Vec<TypedExpr> },
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

impl fmt::Display for TopLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TopLevel::Class(class) => write!(f, "class {} {{ ... }}", class.name),
            TopLevel::Stmt(stmt) => write!(f, "{}", stmt),
        }
    }
}

impl fmt::Display for ClassDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "class {} {{", self.name)?;
        for field in &self.fields {
            writeln!(f, "    {}: {}", field.name, field.ty)?;
        }
        for method in &self.methods {
            writeln!(f, "    def {}(...) {{ ... }}", method.name)?;
        }
        write!(f, "}}")
    }
}

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
                target,
                type_ann,
                value,
            } => {
                let target_str = match target {
                    AssignTarget::Var(name) => name.clone(),
                    AssignTarget::Field { object, field } => format!("{}.{}", object, field),
                };
                if let Some(t) = type_ann {
                    write!(f, "{}: {} = {}", target_str, t, value)
                } else {
                    write!(f, "{} = {}", target_str, value)
                }
            }
            Stmt::Delete(expr) => write!(f, "delete {}", expr),
            Stmt::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

impl fmt::Display for TypedExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Int(n) => write!(f, "{}", n),
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Var(name) => write!(f, "{}", name),
            Expr::SelfRef => write!(f, "self"),
            Expr::Unary { op, expr } => write!(f, "({}{})", op, expr.expr),
            Expr::Binary { op, left, right } => {
                write!(f, "({} {} {})", left.expr, op, right.expr)
            }
            Expr::Call { name, args } => {
                let args_str: Vec<_> = args.iter().map(|a| a.expr.to_string()).collect();
                write!(f, "{}({})", name, args_str.join(", "))
            }
            Expr::MethodCall {
                object,
                method,
                args,
            } => {
                let args_str: Vec<_> = args.iter().map(|a| a.expr.to_string()).collect();
                write!(f, "{}.{}({})", object.expr, method, args_str.join(", "))
            }
            Expr::FieldAccess { object, field } => {
                write!(f, "{}.{}", object.expr, field)
            }
            Expr::New { class, args } => {
                let args_str: Vec<_> = args.iter().map(|a| a.expr.to_string()).collect();
                write!(f, "new {}({})", class, args_str.join(", "))
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
