//! Visitor Pattern for AST Traversal
//!
//! The visitor pattern separates algorithms from the data structures they operate on.
//! This makes it easy to add new operations without modifying the AST.
//!
//! ## Example Visitors
//!
//! - `PrettyPrinter`: Display AST in readable format
//! - `ConstantFolder`: Evaluate constant expressions at compile time
//! - `AlgebraicSimplifier`: Apply algebraic identities (x + 0 = x, etc.)

use crate::ast::{BinaryOp, Expr, Stmt, TypedExpr, UnaryOp};

// ANCHOR: expr_visitor
/// Visitor trait for traversing typed expressions
///
/// Each visit method returns the transformed expression.
/// Default implementations traverse children recursively.
pub trait ExprVisitor {
    /// Visit any expression - dispatches to specific visit methods
    fn visit_expr(&mut self, expr: &TypedExpr) -> TypedExpr {
        let new_expr = match &expr.expr {
            Expr::Int(n) => self.visit_int(*n),
            Expr::Bool(b) => self.visit_bool(*b),
            Expr::Var(name) => self.visit_var(name),
            Expr::Unary { op, expr: inner } => self.visit_unary(*op, inner),
            Expr::Binary { op, left, right } => self.visit_binary(*op, left, right),
            Expr::Call { name, args } => self.visit_call(name, args),
            Expr::If {
                cond,
                then_branch,
                else_branch,
            } => self.visit_if(cond, then_branch, else_branch),
            Expr::While { cond, body } => self.visit_while(cond, body),
            Expr::Block(stmts) => self.visit_block(stmts),
        };
        TypedExpr {
            expr: new_expr,
            ty: expr.ty.clone(),
        }
    }

    fn visit_int(&mut self, n: i64) -> Expr {
        Expr::Int(n)
    }

    fn visit_bool(&mut self, b: bool) -> Expr {
        Expr::Bool(b)
    }

    fn visit_var(&mut self, name: &str) -> Expr {
        Expr::Var(name.to_string())
    }

    fn visit_unary(&mut self, op: UnaryOp, expr: &TypedExpr) -> Expr {
        let visited = self.visit_expr(expr);
        Expr::Unary {
            op,
            expr: Box::new(visited),
        }
    }

    fn visit_binary(&mut self, op: BinaryOp, left: &TypedExpr, right: &TypedExpr) -> Expr {
        let l = self.visit_expr(left);
        let r = self.visit_expr(right);
        Expr::Binary {
            op,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    fn visit_call(&mut self, name: &str, args: &[TypedExpr]) -> Expr {
        let visited_args: Vec<TypedExpr> = args.iter().map(|a| self.visit_expr(a)).collect();
        Expr::Call {
            name: name.to_string(),
            args: visited_args,
        }
    }

    fn visit_if(&mut self, cond: &TypedExpr, then_branch: &[Stmt], else_branch: &[Stmt]) -> Expr {
        let visited_cond = self.visit_expr(cond);
        let visited_then: Vec<Stmt> = then_branch.iter().map(|s| self.visit_stmt(s)).collect();
        let visited_else: Vec<Stmt> = else_branch.iter().map(|s| self.visit_stmt(s)).collect();
        Expr::If {
            cond: Box::new(visited_cond),
            then_branch: visited_then,
            else_branch: visited_else,
        }
    }

    fn visit_while(&mut self, cond: &TypedExpr, body: &[Stmt]) -> Expr {
        let visited_cond = self.visit_expr(cond);
        let visited_body: Vec<Stmt> = body.iter().map(|s| self.visit_stmt(s)).collect();
        Expr::While {
            cond: Box::new(visited_cond),
            body: visited_body,
        }
    }

    fn visit_block(&mut self, stmts: &[Stmt]) -> Expr {
        let visited: Vec<Stmt> = stmts.iter().map(|s| self.visit_stmt(s)).collect();
        Expr::Block(visited)
    }

    /// Visit a statement
    fn visit_stmt(&mut self, stmt: &Stmt) -> Stmt {
        match stmt {
            Stmt::Function {
                name,
                params,
                return_type,
                body,
            } => {
                let visited_body: Vec<Stmt> = body.iter().map(|s| self.visit_stmt(s)).collect();
                Stmt::Function {
                    name: name.clone(),
                    params: params.clone(),
                    return_type: return_type.clone(),
                    body: visited_body,
                }
            }
            Stmt::Return(expr) => Stmt::Return(self.visit_expr(expr)),
            Stmt::Assignment {
                name,
                type_ann,
                value,
            } => Stmt::Assignment {
                name: name.clone(),
                type_ann: type_ann.clone(),
                value: self.visit_expr(value),
            },
            Stmt::Expr(expr) => Stmt::Expr(self.visit_expr(expr)),
        }
    }
}
// ANCHOR_END: expr_visitor

// =============================================================================
// Pretty Printer - Displays AST in readable format
// =============================================================================

/// Pretty prints the AST with indentation
pub struct PrettyPrinter {
    indent: usize,
    output: String,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter {
            indent: 0,
            output: String::new(),
        }
    }

    pub fn print_program(stmts: &[Stmt]) -> String {
        let mut printer = PrettyPrinter::new();
        for stmt in stmts {
            printer.print_stmt(stmt);
            printer.output.push('\n');
        }
        printer.output
    }

    fn indent_str(&self) -> String {
        "  ".repeat(self.indent)
    }

    fn print_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Function {
                name,
                params,
                return_type,
                body,
            } => {
                let params_str: Vec<String> = params
                    .iter()
                    .map(|(n, t)| format!("{}: {}", n, t))
                    .collect();
                self.output.push_str(&format!(
                    "{}def {}({}) -> {} {{\n",
                    self.indent_str(),
                    name,
                    params_str.join(", "),
                    return_type
                ));
                self.indent += 1;
                for s in body {
                    self.print_stmt(s);
                    self.output.push('\n');
                }
                self.indent -= 1;
                self.output.push_str(&format!("{}}}", self.indent_str()));
            }
            Stmt::Return(expr) => {
                self.output.push_str(&format!(
                    "{}return {}",
                    self.indent_str(),
                    self.format_expr(expr)
                ));
            }
            Stmt::Assignment {
                name,
                type_ann,
                value,
            } => {
                let type_str = type_ann
                    .as_ref()
                    .map(|t| format!(": {}", t))
                    .unwrap_or_default();
                self.output.push_str(&format!(
                    "{}{}{} = {}",
                    self.indent_str(),
                    name,
                    type_str,
                    self.format_expr(value)
                ));
            }
            Stmt::Expr(expr) => {
                self.output
                    .push_str(&format!("{}{}", self.indent_str(), self.format_expr(expr)));
            }
        }
    }

    fn format_expr(&self, expr: &TypedExpr) -> String {
        match &expr.expr {
            Expr::Int(n) => n.to_string(),
            Expr::Bool(b) => b.to_string(),
            Expr::Var(name) => name.clone(),
            Expr::Unary { op, expr } => format!("{}{}", op, self.format_expr(expr)),
            Expr::Binary { op, left, right } => {
                format!(
                    "({} {} {})",
                    self.format_expr(left),
                    op,
                    self.format_expr(right)
                )
            }
            Expr::Call { name, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.format_expr(a)).collect();
                format!("{}({})", name, args_str.join(", "))
            }
            Expr::If { cond, .. } => {
                format!("if ({}) {{ ... }} else {{ ... }}", self.format_expr(cond))
            }
            Expr::While { cond, .. } => format!("while ({}) {{ ... }}", self.format_expr(cond)),
            Expr::Block(_) => "{ ... }".to_string(),
        }
    }
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Constant Folder - Evaluates constant expressions at compile time
// =============================================================================

// ANCHOR: constant_folder
/// Folds constant expressions: `1 + 2` becomes `3`
///
/// This is a simple optimization that evaluates expressions
/// where all operands are known at compile time.
pub struct ConstantFolder;

impl ConstantFolder {
    pub fn new() -> Self {
        ConstantFolder
    }

    pub fn fold_program(stmts: &[Stmt]) -> Vec<Stmt> {
        let mut folder = ConstantFolder::new();
        stmts.iter().map(|s| folder.visit_stmt(s)).collect()
    }
}

impl Default for ConstantFolder {
    fn default() -> Self {
        Self::new()
    }
}

impl ExprVisitor for ConstantFolder {
    fn visit_binary(&mut self, op: BinaryOp, left: &TypedExpr, right: &TypedExpr) -> Expr {
        // First, recursively fold children
        let l = self.visit_expr(left);
        let r = self.visit_expr(right);

        // Try to fold if both are constants
        if let (Expr::Int(lv), Expr::Int(rv)) = (&l.expr, &r.expr) {
            let result = match op {
                BinaryOp::Add => Some(lv + rv),
                BinaryOp::Sub => Some(lv - rv),
                BinaryOp::Mul => Some(lv * rv),
                BinaryOp::Div if *rv != 0 => Some(lv / rv),
                BinaryOp::Mod if *rv != 0 => Some(lv % rv),
                _ => None,
            };
            if let Some(val) = result {
                return Expr::Int(val);
            }
        }

        // Try boolean constant folding for comparisons
        if let (Expr::Int(lv), Expr::Int(rv)) = (&l.expr, &r.expr) {
            let result = match op {
                BinaryOp::Lt => Some(*lv < *rv),
                BinaryOp::Gt => Some(*lv > *rv),
                BinaryOp::Le => Some(*lv <= *rv),
                BinaryOp::Ge => Some(*lv >= *rv),
                BinaryOp::Eq => Some(*lv == *rv),
                BinaryOp::Ne => Some(*lv != *rv),
                _ => None,
            };
            if let Some(val) = result {
                return Expr::Bool(val);
            }
        }

        // Can't fold, return as-is
        Expr::Binary {
            op,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    fn visit_unary(&mut self, op: UnaryOp, expr: &TypedExpr) -> Expr {
        let e = self.visit_expr(expr);

        match (&op, &e.expr) {
            (UnaryOp::Neg, Expr::Int(n)) => Expr::Int(-n),
            (UnaryOp::Not, Expr::Bool(b)) => Expr::Bool(!b),
            _ => Expr::Unary {
                op,
                expr: Box::new(e),
            },
        }
    }
}
// ANCHOR_END: constant_folder

// =============================================================================
// Algebraic Simplifier - Applies algebraic identities
// =============================================================================

// ANCHOR: algebraic_simplifier
/// Applies algebraic simplifications:
/// - `x + 0` → `x`
/// - `x - 0` → `x`
/// - `x * 0` → `0`
/// - `x * 1` → `x`
/// - `x / 1` → `x`
/// - `0 + x` → `x`
/// - `1 * x` → `x`
/// - `0 * x` → `0`
pub struct AlgebraicSimplifier;

impl AlgebraicSimplifier {
    pub fn new() -> Self {
        AlgebraicSimplifier
    }

    pub fn simplify_program(stmts: &[Stmt]) -> Vec<Stmt> {
        let mut simplifier = AlgebraicSimplifier::new();
        stmts.iter().map(|s| simplifier.visit_stmt(s)).collect()
    }
}

impl Default for AlgebraicSimplifier {
    fn default() -> Self {
        Self::new()
    }
}

impl ExprVisitor for AlgebraicSimplifier {
    fn visit_binary(&mut self, op: BinaryOp, left: &TypedExpr, right: &TypedExpr) -> Expr {
        // First, recursively simplify children
        let l = self.visit_expr(left);
        let r = self.visit_expr(right);

        // Apply algebraic identities
        match (&op, &l.expr, &r.expr) {
            // x + 0 = x
            (BinaryOp::Add, _, Expr::Int(0)) => return l.expr,
            // 0 + x = x
            (BinaryOp::Add, Expr::Int(0), _) => return r.expr,
            // x - 0 = x
            (BinaryOp::Sub, _, Expr::Int(0)) => return l.expr,
            // x * 0 = 0
            (BinaryOp::Mul, _, Expr::Int(0)) => return Expr::Int(0),
            // 0 * x = 0
            (BinaryOp::Mul, Expr::Int(0), _) => return Expr::Int(0),
            // x * 1 = x
            (BinaryOp::Mul, _, Expr::Int(1)) => return l.expr,
            // 1 * x = x
            (BinaryOp::Mul, Expr::Int(1), _) => return r.expr,
            // x / 1 = x
            (BinaryOp::Div, _, Expr::Int(1)) => return l.expr,
            _ => {}
        }

        Expr::Binary {
            op,
            left: Box::new(l),
            right: Box::new(r),
        }
    }
}
// ANCHOR_END: algebraic_simplifier

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::typeck::typecheck;

    fn parse_and_check(source: &str) -> Vec<Stmt> {
        let mut program = parse(source).unwrap();
        typecheck(&mut program).unwrap();
        program
    }

    #[test]
    fn test_pretty_printer() {
        let program = parse_and_check("def add(a: int, b: int) -> int { return a + b }");
        let output = PrettyPrinter::print_program(&program);
        assert!(output.contains("def add"));
        assert!(output.contains("return"));
    }

    #[test]
    fn test_constant_folding_arithmetic() {
        let program = parse_and_check("def test() -> int { return 1 + 2 * 3 }");
        let folded = ConstantFolder::fold_program(&program);

        // After folding: 1 + 2 * 3 should become 7
        if let Stmt::Function { body, .. } = &folded[0] {
            if let Stmt::Return(expr) = &body[0] {
                assert_eq!(expr.expr, Expr::Int(7));
            }
        }
    }

    #[test]
    fn test_constant_folding_comparison() {
        let program = parse_and_check("def test() -> bool { return 5 < 10 }");
        let folded = ConstantFolder::fold_program(&program);

        if let Stmt::Function { body, .. } = &folded[0] {
            if let Stmt::Return(expr) = &body[0] {
                assert_eq!(expr.expr, Expr::Bool(true));
            }
        }
    }

    #[test]
    fn test_algebraic_simplification_add_zero() {
        let program = parse_and_check("def test(x: int) -> int { return x + 0 }");
        let simplified = AlgebraicSimplifier::simplify_program(&program);

        // x + 0 should become x
        if let Stmt::Function { body, .. } = &simplified[0] {
            if let Stmt::Return(expr) = &body[0] {
                assert_eq!(expr.expr, Expr::Var("x".to_string()));
            }
        }
    }

    #[test]
    fn test_algebraic_simplification_mul_zero() {
        let program = parse_and_check("def test(x: int) -> int { return x * 0 }");
        let simplified = AlgebraicSimplifier::simplify_program(&program);

        // x * 0 should become 0
        if let Stmt::Function { body, .. } = &simplified[0] {
            if let Stmt::Return(expr) = &body[0] {
                assert_eq!(expr.expr, Expr::Int(0));
            }
        }
    }

    #[test]
    fn test_algebraic_simplification_mul_one() {
        let program = parse_and_check("def test(x: int) -> int { return x * 1 }");
        let simplified = AlgebraicSimplifier::simplify_program(&program);

        // x * 1 should become x
        if let Stmt::Function { body, .. } = &simplified[0] {
            if let Stmt::Return(expr) = &body[0] {
                assert_eq!(expr.expr, Expr::Var("x".to_string()));
            }
        }
    }

    #[test]
    fn test_combined_optimizations() {
        // First fold constants, then simplify
        let program = parse_and_check("def test(x: int) -> int { return x * (1 + 0) }");

        // 1. Constant fold: 1 + 0 = 1
        let folded = ConstantFolder::fold_program(&program);

        // 2. Algebraic simplify: x * 1 = x
        let simplified = AlgebraicSimplifier::simplify_program(&folded);

        if let Stmt::Function { body, .. } = &simplified[0] {
            if let Stmt::Return(expr) = &body[0] {
                assert_eq!(expr.expr, Expr::Var("x".to_string()));
            }
        }
    }
}
