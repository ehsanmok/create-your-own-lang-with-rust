//! Visitor Pattern for AST Traversal
//!
//! The visitor pattern separates algorithms from the data structures they operate on.
//! Extended from Secondlang to handle class definitions and new expressions.

use crate::ast::{
    AssignTarget, BinaryOp, ClassDef, Expr, MethodDef, Stmt, TopLevel, TypedExpr, UnaryOp,
};

// ANCHOR: expr_visitor
/// Visitor trait for traversing typed expressions
pub trait ExprVisitor {
    /// Visit any expression - dispatches to specific visit methods
    fn visit_expr(&mut self, expr: &TypedExpr) -> TypedExpr {
        let new_expr = match &expr.expr {
            Expr::Int(n) => self.visit_int(*n),
            Expr::Bool(b) => self.visit_bool(*b),
            Expr::Var(name) => self.visit_var(name),
            Expr::SelfRef => self.visit_self(),
            Expr::Unary { op, expr: inner } => self.visit_unary(*op, inner),
            Expr::Binary { op, left, right } => self.visit_binary(*op, left, right),
            Expr::Call { name, args } => self.visit_call(name, args),
            Expr::MethodCall {
                object,
                method,
                args,
            } => self.visit_method_call(object, method, args),
            Expr::FieldAccess { object, field } => self.visit_field_access(object, field),
            Expr::New { class, args } => self.visit_new(class, args),
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

    fn visit_self(&mut self) -> Expr {
        Expr::SelfRef
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

    fn visit_method_call(&mut self, object: &TypedExpr, method: &str, args: &[TypedExpr]) -> Expr {
        let visited_object = self.visit_expr(object);
        let visited_args: Vec<TypedExpr> = args.iter().map(|a| self.visit_expr(a)).collect();
        Expr::MethodCall {
            object: Box::new(visited_object),
            method: method.to_string(),
            args: visited_args,
        }
    }

    fn visit_field_access(&mut self, object: &TypedExpr, field: &str) -> Expr {
        let visited_object = self.visit_expr(object);
        Expr::FieldAccess {
            object: Box::new(visited_object),
            field: field.to_string(),
        }
    }

    fn visit_new(&mut self, class: &str, args: &[TypedExpr]) -> Expr {
        let visited_args: Vec<TypedExpr> = args.iter().map(|a| self.visit_expr(a)).collect();
        Expr::New {
            class: class.to_string(),
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
                target,
                type_ann,
                value,
            } => {
                let visited_target = match target {
                    AssignTarget::Var(name) => AssignTarget::Var(name.clone()),
                    AssignTarget::Field { object, field } => AssignTarget::Field {
                        object: Box::new(self.visit_expr(object)),
                        field: field.clone(),
                    },
                };
                Stmt::Assignment {
                    target: visited_target,
                    type_ann: type_ann.clone(),
                    value: self.visit_expr(value),
                }
            }
            Stmt::Delete(expr) => Stmt::Delete(self.visit_expr(expr)),
            Stmt::Expr(expr) => Stmt::Expr(self.visit_expr(expr)),
        }
    }

    /// Visit a top-level item
    fn visit_top_level(&mut self, item: &TopLevel) -> TopLevel {
        match item {
            TopLevel::Class(class) => TopLevel::Class(self.visit_class(class)),
            TopLevel::Stmt(stmt) => TopLevel::Stmt(self.visit_stmt(stmt)),
        }
    }

    /// Visit a class definition
    fn visit_class(&mut self, class: &ClassDef) -> ClassDef {
        let visited_methods: Vec<MethodDef> =
            class.methods.iter().map(|m| self.visit_method(m)).collect();

        ClassDef {
            name: class.name.clone(),
            fields: class.fields.clone(),
            methods: visited_methods,
        }
    }

    /// Visit a method definition
    fn visit_method(&mut self, method: &MethodDef) -> MethodDef {
        let visited_body: Vec<Stmt> = method.body.iter().map(|s| self.visit_stmt(s)).collect();

        MethodDef {
            name: method.name.clone(),
            params: method.params.clone(),
            return_type: method.return_type.clone(),
            body: visited_body,
        }
    }
}
// ANCHOR_END: expr_visitor

// =============================================================================
// Pretty Printer
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

    pub fn print_program(items: &[TopLevel]) -> String {
        let mut printer = PrettyPrinter::new();
        for item in items {
            printer.print_top_level(item);
            printer.output.push('\n');
        }
        printer.output
    }

    fn indent_str(&self) -> String {
        "  ".repeat(self.indent)
    }

    fn print_top_level(&mut self, item: &TopLevel) {
        match item {
            TopLevel::Class(class) => self.print_class(class),
            TopLevel::Stmt(stmt) => self.print_stmt(stmt),
        }
    }

    fn print_class(&mut self, class: &ClassDef) {
        self.output
            .push_str(&format!("{}class {} {{\n", self.indent_str(), class.name));
        self.indent += 1;

        for field in &class.fields {
            self.output.push_str(&format!(
                "{}{}: {}\n",
                self.indent_str(),
                field.name,
                field.ty
            ));
        }

        for method in &class.methods {
            self.print_method(method);
            self.output.push('\n');
        }

        self.indent -= 1;
        self.output.push_str(&format!("{}}}", self.indent_str()));
    }

    fn print_method(&mut self, method: &MethodDef) {
        let params_str: Vec<String> = method
            .params
            .iter()
            .map(|(n, t)| format!("{}: {}", n, t))
            .collect();
        self.output.push_str(&format!(
            "{}def {}(self{}{}) -> {} {{\n",
            self.indent_str(),
            method.name,
            if params_str.is_empty() { "" } else { ", " },
            params_str.join(", "),
            method.return_type
        ));
        self.indent += 1;
        for s in &method.body {
            self.print_stmt(s);
            self.output.push('\n');
        }
        self.indent -= 1;
        self.output.push_str(&format!("{}}}", self.indent_str()));
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
                target,
                type_ann,
                value,
            } => {
                let target_str = match target {
                    AssignTarget::Var(name) => name.clone(),
                    AssignTarget::Field { object, field } => {
                        format!("{}.{}", self.format_expr(object), field)
                    }
                };
                let type_str = type_ann
                    .as_ref()
                    .map(|t| format!(": {}", t))
                    .unwrap_or_default();
                self.output.push_str(&format!(
                    "{}{}{} = {}",
                    self.indent_str(),
                    target_str,
                    type_str,
                    self.format_expr(value)
                ));
            }
            Stmt::Delete(expr) => {
                self.output.push_str(&format!(
                    "{}delete {}",
                    self.indent_str(),
                    self.format_expr(expr)
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
            Expr::SelfRef => "self".to_string(),
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
            Expr::MethodCall {
                object,
                method,
                args,
            } => {
                let args_str: Vec<String> = args.iter().map(|a| self.format_expr(a)).collect();
                format!(
                    "{}.{}({})",
                    self.format_expr(object),
                    method,
                    args_str.join(", ")
                )
            }
            Expr::FieldAccess { object, field } => {
                format!("{}.{}", self.format_expr(object), field)
            }
            Expr::New { class, args } => {
                let args_str: Vec<String> = args.iter().map(|a| self.format_expr(a)).collect();
                format!("new {}({})", class, args_str.join(", "))
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
// Constant Folder (same as Secondlang)
// =============================================================================

/// Folds constant expressions: `1 + 2` becomes `3`
pub struct ConstantFolder;

impl ConstantFolder {
    pub fn new() -> Self {
        ConstantFolder
    }

    pub fn fold_program(items: &[TopLevel]) -> Vec<TopLevel> {
        let mut folder = ConstantFolder::new();
        items.iter().map(|i| folder.visit_top_level(i)).collect()
    }
}

impl Default for ConstantFolder {
    fn default() -> Self {
        Self::new()
    }
}

impl ExprVisitor for ConstantFolder {
    fn visit_binary(&mut self, op: BinaryOp, left: &TypedExpr, right: &TypedExpr) -> Expr {
        let l = self.visit_expr(left);
        let r = self.visit_expr(right);

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

        // Try boolean constant folding
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::typeck::typecheck;

    fn parse_and_check(source: &str) -> Vec<TopLevel> {
        let mut program = parse(source).unwrap();
        typecheck(&mut program).unwrap();
        program
    }

    #[test]
    fn test_pretty_printer_class() {
        let program = parse_and_check(
            r#"
            class Point {
                x: int
                def get_x(self) -> int { return self.x }
            }
        "#,
        );
        let output = PrettyPrinter::print_program(&program);
        assert!(output.contains("class Point"));
        assert!(output.contains("x: int"));
        assert!(output.contains("def get_x"));
    }

    #[test]
    fn test_constant_folding() {
        let program = parse_and_check("def test() -> int { return 1 + 2 * 3 }");
        let folded = ConstantFolder::fold_program(&program);

        if let TopLevel::Stmt(Stmt::Function { body, .. }) = &folded[0] {
            if let Stmt::Return(expr) = &body[0] {
                assert_eq!(expr.expr, Expr::Int(7));
            }
        }
    }
}
