//! Type Checker and Type Inference for Thirdlang
//!
//! Extends Secondlang's type checker with:
//! - Class type registration
//! - Method and field type checking
//! - Constructor/destructor validation
//! - Self type binding in methods

use std::collections::HashMap;

use crate::ast::{
    AssignTarget, BinaryOp, ClassDef, Expr, MethodDef, Program, Stmt, TopLevel, TypedExpr, UnaryOp,
};
use crate::types::{ClassInfo, ClassRegistry, MethodInfo, Type};

/// Type environment - maps variable names to their types
pub type TypeEnv = HashMap<String, Type>;

/// Context for type checking
pub struct TypeContext {
    /// Global type environment (functions)
    pub global_env: TypeEnv,
    /// Class registry
    pub classes: ClassRegistry,
    /// Current class being type-checked (if any)
    pub current_class: Option<String>,
    /// Current method return type (if any)
    pub current_return_type: Option<Type>,
}

impl TypeContext {
    pub fn new() -> Self {
        TypeContext {
            global_env: TypeEnv::new(),
            classes: ClassRegistry::new(),
            current_class: None,
            current_return_type: None,
        }
    }
}

impl Default for TypeContext {
    fn default() -> Self {
        Self::new()
    }
}

// ANCHOR: typecheck
/// Type check and infer types for a program
pub fn typecheck(program: &mut Program) -> Result<ClassRegistry, String> {
    let mut ctx = TypeContext::new();

    // First pass: register all classes
    for item in program.iter() {
        if let TopLevel::Class(class) = item {
            register_class(&mut ctx, class)?;
        }
    }

    // Second pass: collect function signatures
    for item in program.iter() {
        if let TopLevel::Stmt(Stmt::Function {
            name,
            params,
            return_type,
            ..
        }) = item
        {
            let param_types: Vec<Type> = params.iter().map(|(_, t)| t.clone()).collect();
            let func_type = Type::Function {
                params: param_types,
                ret: Box::new(return_type.clone()),
            };
            ctx.global_env.insert(name.clone(), func_type);
        }
    }

    // Third pass: type check classes
    for item in program.iter_mut() {
        if let TopLevel::Class(class) = item {
            typecheck_class(&mut ctx, class)?;
        }
    }

    // Fourth pass: type check statements
    // Use a persistent environment for top-level statements
    let mut top_level_env = ctx.global_env.clone();
    for item in program.iter_mut() {
        if let TopLevel::Stmt(stmt) = item {
            typecheck_stmt(&mut ctx, stmt, &mut top_level_env)?;
        }
    }

    Ok(ctx.classes)
}
// ANCHOR_END: typecheck

/// Register a class in the class registry
fn register_class(ctx: &mut TypeContext, class: &ClassDef) -> Result<(), String> {
    let mut class_info = ClassInfo::new(class.name.clone());

    // Register fields
    for field in &class.fields {
        // Validate field type
        validate_type(ctx, &field.ty)?;
        class_info.add_field(field.name.clone(), field.ty.clone());
    }

    // Register methods
    for method in &class.methods {
        let method_info = MethodInfo {
            name: method.name.clone(),
            params: method.params.clone(),
            return_type: method.return_type.clone(),
            is_constructor: method.is_constructor(),
            is_destructor: method.is_destructor(),
        };
        class_info.add_method(method_info);
    }

    ctx.classes.insert(class.name.clone(), class_info);
    Ok(())
}

/// Validate that a type is valid (class exists if it's a class type)
fn validate_type(ctx: &TypeContext, ty: &Type) -> Result<(), String> {
    if let Type::Class(name) = ty {
        if !ctx.classes.contains_key(name) {
            return Err(format!("Unknown class: {}", name));
        }
    }
    Ok(())
}

/// Type check a class definition
fn typecheck_class(ctx: &mut TypeContext, class: &mut ClassDef) -> Result<(), String> {
    ctx.current_class = Some(class.name.clone());

    for method in &mut class.methods {
        typecheck_method(ctx, &class.name, method)?;
    }

    ctx.current_class = None;
    Ok(())
}

/// Type check a method
fn typecheck_method(
    ctx: &mut TypeContext,
    class_name: &str,
    method: &mut MethodDef,
) -> Result<(), String> {
    // Create local environment with parameters
    let mut local_env = ctx.global_env.clone();

    // Add 'self' as the class type
    local_env.insert("self".to_string(), Type::Class(class_name.to_string()));

    // Add method parameters
    for (param_name, param_type) in &method.params {
        validate_type(ctx, param_type)?;
        local_env.insert(param_name.clone(), param_type.clone());
    }

    // Set expected return type
    ctx.current_return_type = Some(method.return_type.clone());

    // Type check body
    for stmt in &mut method.body {
        typecheck_stmt(ctx, stmt, &mut local_env)?;
    }

    ctx.current_return_type = None;
    Ok(())
}

fn typecheck_stmt(
    ctx: &mut TypeContext,
    stmt: &mut Stmt,
    env: &mut TypeEnv,
) -> Result<Type, String> {
    match stmt {
        Stmt::Function {
            name: _,
            params,
            return_type,
            body,
        } => {
            // Create local environment with parameters
            let mut local_env = env.clone();
            for (param_name, param_type) in params.iter() {
                local_env.insert(param_name.clone(), param_type.clone());
            }

            ctx.current_return_type = Some(return_type.clone());

            // Type check body
            for body_stmt in body.iter_mut() {
                typecheck_stmt(ctx, body_stmt, &mut local_env)?;
            }

            ctx.current_return_type = None;
            Ok(Type::Unit)
        }

        Stmt::Return(expr) => {
            typecheck_expr(ctx, expr, env)?;

            // Check return type matches
            if let Some(expected) = &ctx.current_return_type {
                if *expected != Type::Unknown && *expected != Type::Unit {
                    let _ = expected.unify(&expr.ty)?;
                }
            }

            Ok(expr.ty.clone())
        }

        Stmt::Assignment {
            target,
            type_ann,
            value,
        } => {
            typecheck_expr(ctx, value, env)?;

            match target {
                AssignTarget::Var(name) => {
                    let var_type = if let Some(ann) = type_ann {
                        validate_type(ctx, ann)?;
                        let _ = ann.unify(&value.ty)?;
                        ann.clone()
                    } else if let Some(existing) = env.get(name) {
                        // Variable already exists - check type matches
                        let _ = existing.unify(&value.ty)?;
                        existing.clone()
                    } else {
                        // Infer from value
                        value.ty.clone()
                    };

                    env.insert(name.clone(), var_type.clone());
                    Ok(var_type)
                }
                AssignTarget::Field { object, field } => {
                    typecheck_expr(ctx, object, env)?;

                    // Get the class info
                    let class_name = object.ty.class_name().ok_or_else(|| {
                        format!("Cannot access field on non-class type: {}", object.ty)
                    })?;

                    let class_info = ctx
                        .classes
                        .get(class_name)
                        .ok_or_else(|| format!("Unknown class: {}", class_name))?;

                    let field_type = class_info.get_field(field).ok_or_else(|| {
                        format!("Unknown field {} on class {}", field, class_name)
                    })?;

                    let _ = field_type.unify(&value.ty)?;
                    Ok(field_type.clone())
                }
            }
        }

        Stmt::Delete(expr) => {
            typecheck_expr(ctx, expr, env)?;

            // Can only delete class instances
            if !expr.ty.is_class() {
                return Err(format!("Cannot delete non-class type: {}", expr.ty));
            }

            Ok(Type::Unit)
        }

        Stmt::Expr(expr) => {
            typecheck_expr(ctx, expr, env)?;
            Ok(expr.ty.clone())
        }
    }
}

// ANCHOR: typecheck_expr
fn typecheck_expr(
    ctx: &mut TypeContext,
    expr: &mut TypedExpr,
    env: &TypeEnv,
) -> Result<(), String> {
    match &mut expr.expr {
        Expr::Int(_) => {
            expr.ty = Type::Int;
        }

        Expr::Bool(_) => {
            expr.ty = Type::Bool;
        }

        Expr::Var(name) => {
            if let Some(ty) = env.get(name) {
                expr.ty = ty.clone();
            } else {
                return Err(format!("Undefined variable: {}", name));
            }
        }

        Expr::SelfRef => {
            if let Some(class_name) = &ctx.current_class {
                expr.ty = Type::Class(class_name.clone());
            } else {
                return Err("'self' can only be used inside a method".to_string());
            }
        }

        Expr::Unary { op, expr: inner } => {
            typecheck_expr(ctx, inner, env)?;
            match op {
                UnaryOp::Neg => {
                    if inner.ty != Type::Int {
                        return Err(format!("Cannot negate non-integer type: {}", inner.ty));
                    }
                    expr.ty = Type::Int;
                }
                UnaryOp::Not => {
                    if inner.ty != Type::Bool {
                        return Err(format!("Cannot negate non-boolean type: {}", inner.ty));
                    }
                    expr.ty = Type::Bool;
                }
            }
        }

        Expr::Binary { op, left, right } => {
            typecheck_expr(ctx, left, env)?;
            typecheck_expr(ctx, right, env)?;

            match op {
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                    if left.ty != Type::Int || right.ty != Type::Int {
                        return Err(format!(
                            "Arithmetic operation requires int operands, got {} and {}",
                            left.ty, right.ty
                        ));
                    }
                    expr.ty = Type::Int;
                }
                BinaryOp::Lt | BinaryOp::Gt | BinaryOp::Le | BinaryOp::Ge => {
                    if left.ty != Type::Int || right.ty != Type::Int {
                        return Err(format!(
                            "Comparison requires int operands, got {} and {}",
                            left.ty, right.ty
                        ));
                    }
                    expr.ty = Type::Bool;
                }
                BinaryOp::Eq | BinaryOp::Ne => {
                    let _ = left.ty.unify(&right.ty)?;
                    expr.ty = Type::Bool;
                }
            }
        }

        Expr::Call { name, args } => {
            // Look up function type
            let func_type = env
                .get(name)
                .or_else(|| ctx.global_env.get(name))
                .ok_or_else(|| format!("Undefined function: {}", name))?
                .clone();

            if let Type::Function { params, ret } = func_type {
                if args.len() != params.len() {
                    return Err(format!(
                        "Function {} expects {} arguments, got {}",
                        name,
                        params.len(),
                        args.len()
                    ));
                }

                for (arg, param_type) in args.iter_mut().zip(params.iter()) {
                    typecheck_expr(ctx, arg, env)?;
                    let _ = arg.ty.unify(param_type)?;
                }

                expr.ty = *ret;
            } else {
                return Err(format!("{} is not a function", name));
            }
        }

        Expr::MethodCall {
            object,
            method,
            args,
        } => {
            typecheck_expr(ctx, object, env)?;

            // Get the class info
            let class_name = object
                .ty
                .class_name()
                .ok_or_else(|| format!("Cannot call method on non-class type: {}", object.ty))?;

            let class_info = ctx
                .classes
                .get(class_name)
                .ok_or_else(|| format!("Unknown class: {}", class_name))?
                .clone();

            let method_info = class_info
                .get_method(method)
                .ok_or_else(|| format!("Unknown method {} on class {}", method, class_name))?
                .clone();

            // Check argument count
            if args.len() != method_info.params.len() {
                return Err(format!(
                    "Method {}.{} expects {} arguments, got {}",
                    class_name,
                    method,
                    method_info.params.len(),
                    args.len()
                ));
            }

            // Type check arguments
            for (arg, (_, param_type)) in args.iter_mut().zip(method_info.params.iter()) {
                typecheck_expr(ctx, arg, env)?;
                let _ = arg.ty.unify(param_type)?;
            }

            expr.ty = method_info.return_type.clone();
        }

        Expr::FieldAccess { object, field } => {
            typecheck_expr(ctx, object, env)?;

            // Get the class info
            let class_name = object
                .ty
                .class_name()
                .ok_or_else(|| format!("Cannot access field on non-class type: {}", object.ty))?;

            let class_info = ctx
                .classes
                .get(class_name)
                .ok_or_else(|| format!("Unknown class: {}", class_name))?;

            let field_type = class_info
                .get_field(field)
                .ok_or_else(|| format!("Unknown field {} on class {}", field, class_name))?;

            expr.ty = field_type.clone();
        }

        Expr::New { class, args } => {
            // Get the class info
            let class_info = ctx
                .classes
                .get(class)
                .ok_or_else(|| format!("Unknown class: {}", class))?
                .clone();

            // Get constructor if exists
            if let Some(ctor) = class_info.get_method("__init__") {
                if args.len() != ctor.params.len() {
                    return Err(format!(
                        "Constructor for {} expects {} arguments, got {}",
                        class,
                        ctor.params.len(),
                        args.len()
                    ));
                }

                for (arg, (_, param_type)) in args.iter_mut().zip(ctor.params.iter()) {
                    typecheck_expr(ctx, arg, env)?;
                    let _ = arg.ty.unify(param_type)?;
                }
            } else if !args.is_empty() {
                return Err(format!(
                    "Class {} has no constructor but {} arguments provided",
                    class,
                    args.len()
                ));
            }

            expr.ty = Type::Class(class.clone());
        }

        Expr::If {
            cond,
            then_branch,
            else_branch,
        } => {
            typecheck_expr(ctx, cond, env)?;
            if cond.ty != Type::Bool {
                return Err(format!("If condition must be bool, got {}", cond.ty));
            }

            let mut then_env = env.clone();
            let mut then_type = Type::Unit;
            for stmt in then_branch.iter_mut() {
                then_type = typecheck_stmt(ctx, stmt, &mut then_env)?;
            }

            let mut else_env = env.clone();
            let mut else_type = Type::Unit;
            for stmt in else_branch.iter_mut() {
                else_type = typecheck_stmt(ctx, stmt, &mut else_env)?;
            }

            expr.ty = then_type.unify(&else_type)?;
        }

        Expr::While { cond, body } => {
            typecheck_expr(ctx, cond, env)?;
            if cond.ty != Type::Bool {
                return Err(format!("While condition must be bool, got {}", cond.ty));
            }

            let mut body_env = env.clone();
            for stmt in body.iter_mut() {
                typecheck_stmt(ctx, stmt, &mut body_env)?;
            }

            expr.ty = Type::Unit;
        }

        Expr::Block(stmts) => {
            let mut block_env = env.clone();
            let mut last_type = Type::Unit;
            for stmt in stmts.iter_mut() {
                last_type = typecheck_stmt(ctx, stmt, &mut block_env)?;
            }
            expr.ty = last_type;
        }
    }

    Ok(())
}
// ANCHOR_END: typecheck_expr

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn typecheck_source(source: &str) -> Result<ClassRegistry, String> {
        let mut program = parse(source)?;
        typecheck(&mut program)
    }

    #[test]
    fn test_typecheck_class() {
        let source = r#"
            class Point {
                x: int
                y: int

                def __init__(self, x: int, y: int) {
                    self.x = x
                    self.y = y
                }

                def get_x(self) -> int {
                    return self.x
                }
            }
        "#;
        let classes = typecheck_source(source).unwrap();
        assert!(classes.contains_key("Point"));
    }

    #[test]
    fn test_typecheck_new_expr() {
        let source = r#"
            class Point {
                x: int
                def __init__(self, x: int) {
                    self.x = x
                }
            }
            p = new Point(42)
        "#;
        typecheck_source(source).unwrap();
    }

    #[test]
    fn test_typecheck_method_call() {
        let source = r#"
            class Counter {
                value: int
                def __init__(self, start: int) {
                    self.value = start
                }
                def get(self) -> int {
                    return self.value
                }
            }
            c = new Counter(10)
            c.get()
        "#;
        typecheck_source(source).unwrap();
    }

    #[test]
    fn test_typecheck_wrong_field_type() {
        let source = r#"
            class Point {
                x: int
                def __init__(self, x: int) {
                    self.x = true
                }
            }
        "#;
        assert!(typecheck_source(source).is_err());
    }

    #[test]
    fn test_typecheck_delete() {
        let source = r#"
            class Point { x: int }
            p = new Point()
            delete p
        "#;
        typecheck_source(source).unwrap();
    }

    #[test]
    fn test_typecheck_delete_non_class() {
        let source = r#"
            x: int = 42
            delete x
        "#;
        assert!(typecheck_source(source).is_err());
    }
}
