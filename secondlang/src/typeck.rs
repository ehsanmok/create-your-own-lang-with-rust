//! Type Checker and Type Inference for Secondlang
//!
//! This module performs:
//! - Type checking: verifying type consistency
//! - Type inference: deducing types for expressions without explicit annotations

use std::collections::HashMap;

use crate::ast::{BinaryOp, Expr, Program, Stmt, TypedExpr, UnaryOp};
use crate::types::Type;

/// Type environment - maps variable names to their types
pub type TypeEnv = HashMap<String, Type>;

// ANCHOR: typecheck
/// Type check and infer types for a program
pub fn typecheck(program: &mut Program) -> Result<(), String> {
    let mut env = TypeEnv::new();

    // First pass: collect function signatures
    for stmt in program.iter() {
        if let Stmt::Function {
            name,
            params,
            return_type,
            ..
        } = stmt
        {
            let param_types: Vec<Type> = params.iter().map(|(_, t)| t.clone()).collect();
            let func_type = Type::Function {
                params: param_types,
                ret: Box::new(return_type.clone()),
            };
            env.insert(name.clone(), func_type);
        }
    }

    // Second pass: type check each statement
    for stmt in program.iter_mut() {
        typecheck_stmt(stmt, &mut env)?;
    }

    Ok(())
}
// ANCHOR_END: typecheck

fn typecheck_stmt(stmt: &mut Stmt, env: &mut TypeEnv) -> Result<Type, String> {
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

            // Type check body
            let mut body_type = Type::Unit;
            for body_stmt in body.iter_mut() {
                body_type = typecheck_stmt(body_stmt, &mut local_env)?;
            }

            // Verify return type matches (if not Unknown)
            if *return_type != Type::Unknown && body_type != Type::Unit {
                let _ = return_type.unify(&body_type)?;
            }

            Ok(Type::Unit)
        }

        Stmt::Return(expr) => {
            typecheck_expr(expr, env)?;
            Ok(expr.ty.clone())
        }

        Stmt::Assignment {
            name,
            type_ann,
            value,
        } => {
            typecheck_expr(value, env)?;

            let var_type = if let Some(ann) = type_ann {
                // Explicit type annotation - check it matches
                let _ = ann.unify(&value.ty)?;
                ann.clone()
            } else {
                // Infer from value
                value.ty.clone()
            };

            env.insert(name.clone(), var_type.clone());
            Ok(var_type)
        }

        Stmt::Expr(expr) => {
            typecheck_expr(expr, env)?;
            Ok(expr.ty.clone())
        }
    }
}

// ANCHOR: typecheck_expr
fn typecheck_expr(expr: &mut TypedExpr, env: &TypeEnv) -> Result<(), String> {
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

        Expr::Unary { op, expr: inner } => {
            typecheck_expr(inner, env)?;
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
            typecheck_expr(left, env)?;
            typecheck_expr(right, env)?;

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
                .ok_or_else(|| format!("Undefined function: {}", name))?
                .clone();

            if let Type::Function { params, ret } = func_type {
                // Check argument count
                if args.len() != params.len() {
                    return Err(format!(
                        "Function {} expects {} arguments, got {}",
                        name,
                        params.len(),
                        args.len()
                    ));
                }

                // Type check each argument
                for (arg, param_type) in args.iter_mut().zip(params.iter()) {
                    typecheck_expr(arg, env)?;
                    let _ = arg.ty.unify(param_type)?;
                }

                expr.ty = *ret;
            } else {
                return Err(format!("{} is not a function", name));
            }
        }

        Expr::If {
            cond,
            then_branch,
            else_branch,
        } => {
            typecheck_expr(cond, env)?;
            if cond.ty != Type::Bool {
                return Err(format!("If condition must be bool, got {}", cond.ty));
            }

            // Type check branches
            let mut then_env = env.clone();
            let mut then_type = Type::Unit;
            for stmt in then_branch.iter_mut() {
                then_type = typecheck_stmt(stmt, &mut then_env)?;
            }

            let mut else_env = env.clone();
            let mut else_type = Type::Unit;
            for stmt in else_branch.iter_mut() {
                else_type = typecheck_stmt(stmt, &mut else_env)?;
            }

            // Branches must have same type
            expr.ty = then_type.unify(&else_type)?;
        }

        Expr::While { cond, body } => {
            typecheck_expr(cond, env)?;
            if cond.ty != Type::Bool {
                return Err(format!("While condition must be bool, got {}", cond.ty));
            }

            let mut body_env = env.clone();
            for stmt in body.iter_mut() {
                typecheck_stmt(stmt, &mut body_env)?;
            }

            expr.ty = Type::Unit;
        }

        Expr::Block(stmts) => {
            let mut block_env = env.clone();
            let mut last_type = Type::Unit;
            for stmt in stmts.iter_mut() {
                last_type = typecheck_stmt(stmt, &mut block_env)?;
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

    fn typecheck_source(source: &str) -> Result<Program, String> {
        let mut program = parse(source)?;
        typecheck(&mut program)?;
        Ok(program)
    }

    #[test]
    fn test_typecheck_literals() {
        let mut program = parse("42").unwrap();
        typecheck(&mut program).unwrap();
        if let Stmt::Expr(expr) = &program[0] {
            assert_eq!(expr.ty, Type::Int);
        }
    }

    #[test]
    fn test_typecheck_arithmetic() {
        let program = typecheck_source("1 + 2 * 3").unwrap();
        if let Stmt::Expr(expr) = &program[0] {
            assert_eq!(expr.ty, Type::Int);
        }
    }

    #[test]
    fn test_typecheck_comparison() {
        let program = typecheck_source("1 < 2").unwrap();
        if let Stmt::Expr(expr) = &program[0] {
            assert_eq!(expr.ty, Type::Bool);
        }
    }

    #[test]
    fn test_typecheck_function() {
        let source = r#"
            def add(a: int, b: int) -> int {
                return a + b
            }
            add(1, 2)
        "#;
        let program = typecheck_source(source).unwrap();
        // Function call should have int type
        if let Stmt::Expr(expr) = &program[1] {
            assert_eq!(expr.ty, Type::Int);
        }
    }

    #[test]
    fn test_typecheck_type_error() {
        let source = "1 + true";
        let result = typecheck_source(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_typecheck_fibonacci() {
        let source = r#"
            def fib(n: int) -> int {
                if (n < 2) {
                    return n
                } else {
                    return fib(n - 1) + fib(n - 2)
                }
            }
            fib(10)
        "#;
        let program = typecheck_source(source).unwrap();
        if let Stmt::Expr(expr) = &program[1] {
            assert_eq!(expr.ty, Type::Int);
        }
    }
}
