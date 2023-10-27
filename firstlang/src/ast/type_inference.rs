use std::collections::HashMap;

use anyhow::bail;

use crate::ast::typed::*;
use crate::Result;

type TypeMap = HashMap<Identifier, Type>;
type Binding = (Identifier, Option<Type>);

pub trait InferTypes {
    fn infer_types(&mut self) -> Result<()>;
    fn infer_local(&mut self) -> Result<()>;
}

impl InferTypes for Expr {
    /// Checks and infers types in place.
    fn infer_types(&mut self) -> Result<()> {
        let env = &mut TypeMap::default();
        self.infer_types_with_env(env)
    }

    fn infer_local(&mut self) -> Result<()> {
        let env = &mut TypeMap::default();
        if let ExprKind::Identifier(ident) = &self.kind {
            env.insert(ident.clone(), self.ty.clone());
        }
        self.infer_locally(env).map(|_| ())
    }
}

/// A trait for updating a type based on types around it.
///
/// This trait is implemented by `Type`.
pub trait PushType {
    /// Push the type of `other` into this `Type` and return if this `Type` changed.
    ///
    /// Pushing the type forces this `Type` to be at least as specific as `other (i.e., the same
    /// type with fewer Unknowns in any subtypes). The function returns an error if an incompatible
    /// type was pushed, and otherwise returns a boolean indicating whether this `Type` changed.
    fn push(&mut self, other: &Self) -> Result<bool>;

    /// Sync this `Type` with `other`, calling push in both directions.
    ///
    /// This function returns an error if an incompatible type was pushed in either direction, and
    /// otherwise returns a boolean indicating whether either `Type` changed.
    fn sync(&mut self, other: &mut Self) -> Result<bool> {
        Ok(self.push(other)? || other.push(self)?)
    }

    /// Sets this `Type` to be `other`.
    ///
    /// This function returns an error if `other` is incompatible with this `Type`.
    fn push_complete(&mut self, other: Self) -> Result<bool>;
}

impl PushType for Type {
    /// Sets this `Type` to be `other`.
    fn push_complete(&mut self, other: Self) -> Result<bool> {
        match other {
            Type::Scalar(_) if *self == Type::Unknown => {
                *self = other;
                Ok(true)
            }
            Type::Scalar(_) if *self == other => Ok(false),
            Type::Function(ref params, ref return_type) if *self == Type::Unknown => {
                *self = other;
                Ok(true)
            }
            Type::Function(ref params, ref return_type) if *self == other => Ok(false),
            Type::Unit if *self == Type::Unknown => {
                *self = other;
                Ok(true)
            }
            Type::Unit if *self == other => Ok(false),
            _ => bail!("Type mismatch: expected {} type but got {}", &other, self),
        }
    }

    /// Push the type of `other` into this `Type` and return if this `Type` changed.
    fn push(&mut self, other: &Self) -> Result<bool> {
        // If the other type is Unknown, we cannot infer anything. If this type is Unknown,
        // copy the other type to this one.
        if *other == Type::Unknown {
            Ok(false)
        } else if *self == Type::Unknown {
            *self = other.clone();
            Ok(true)
        } else {
            // Match this `Type` with `other` to make sure the variant is the same. Then, recursively
            // call `push` on any subtypes.
            match (self, other) {
                (&mut Type::Scalar(ref a), &Type::Scalar(ref b)) if *a == *b => Ok(false),
                (
                    &mut Type::Function(ref mut params, ref mut return_type),
                    &Type::Function(ref other_params, ref other_return_type),
                ) if params.len() == other_params.len() => {
                    let mut changed = false;
                    for (this, other) in params.iter_mut().zip(other_params) {
                        changed |= this.push(other)?;
                    }
                    changed |= return_type.push(&other_return_type)?;
                    Ok(changed)
                }
                (this, other) => {
                    bail!(
                        "Type mismatch in push: expected {} type but got {}",
                        other,
                        this
                    )
                }
            }
        }
    }
}

/// This trait contains additional helper methods that are not exposed outside this module.
trait InferTypesInternal {
    fn infer_types_with_env(&mut self, env: &mut TypeMap) -> Result<()>;
    fn infer_locally(&mut self, env: &mut TypeMap) -> Result<bool>;
    fn infer_up(&mut self, env: &mut TypeMap) -> Result<bool>;
}

impl InferTypesInternal for Expr {
    /// Internal implementation of type inference.
    // fn infer_types_with_env(&mut self, env: &mut TypeMap) -> Result<()> {
    //     loop {
    //         if !self.infer_up(env)? {
    //             if self.partially_typed() {
    //                 return bail!("Could not infer some types");
    //             } else {
    //                 return Ok(());
    //             }
    //         }
    //     }
    // }
    fn infer_types_with_env(&mut self, env: &mut TypeMap) -> Result<()> {
        let max_iterations = 1024;
        let mut previous_state: Option<TypeMap> = None;

        for _ in 0..max_iterations {
            if !self.infer_up(env)? {
                if self.partially_typed() {
                    return bail!("Could not infer some types");
                } else {
                    return Ok(());
                }
            }
            if Some(env.clone()) == previous_state {
                return bail!("Type inference loop detected");
            }
            previous_state = Some(env.clone());
        }
        bail!("Reached maximum type inference iterations without convergence")
    }

    fn infer_locally(&mut self, env: &mut TypeMap) -> Result<bool> {
        match &mut self.kind {
            ExprKind::Identifier(ident) => {
                if let Some(ty) = env.get(ident) {
                    self.ty.push(ty)
                } else {
                    bail!("Identifier {} is not defined", ident)
                }
            }
            ExprKind::Literal(LiteralKind::Int(_)) => {
                self.ty.push_complete(Type::Scalar(ScalarKind::Int))
            }
            ExprKind::Literal(LiteralKind::Bool(_)) => {
                self.ty.push_complete(Type::Scalar(ScalarKind::Bool))
            }
            ExprKind::UnaryExpr(UnaryExpr { ref op, ref child }) => match child.ty {
                Type::Scalar(ref skind) if skind.is_numeric() => self.ty.push(&child.ty),
                Type::Unknown => Ok(false),
                _ => bail!(
                    "Expected numeric type for unary expression. Must be Int type but given: {}{}",
                    op,
                    child
                ),
            },
            ExprKind::BinaryExpr(BinaryExpr {
                op,
                ref mut lhs,
                ref mut rhs,
            }) => {
                // First, sync the left and right types into the elem_type.
                let elem_type = &mut Type::Unknown;
                elem_type.push(&lhs.ty)?;
                elem_type.push(&rhs.ty)?;

                if !op.is_comparison() {
                    elem_type.push(&self.ty)?;
                }

                // Now, attempt to push the elem_type back into left and right to "sync" them.
                let mut changed = lhs.ty.push(elem_type)?;
                changed |= rhs.ty.push(elem_type)?;

                // For comparisons, force the type to be Bool.
                if op.is_comparison() {
                    changed |= self.ty.push_complete(Type::Scalar(ScalarKind::Bool))?
                } else {
                    changed |= self.ty.push(elem_type)?;
                }
                Ok(changed)
            }
            ExprKind::Return(Return { ref mut value }) => {
                let mut changed = false;
                changed |= value.ty.push(&self.ty)?;
                changed |= self.ty.push(&value.ty)?;
                Ok(changed)
            }
            ExprKind::Assignment(Assignment {
                ref mut ident,
                ref mut value,
            }) => {
                let mut changed = false;

                // If the identifier isn't already in the environment, add it.
                // Else, sync its type with the type of the value.
                if let Some(existing_type) = env.get_mut(ident) {
                    changed |= existing_type.sync(&mut value.ty)?;
                } else {
                    env.insert(ident.clone(), value.ty.clone());
                    changed = true; // The environment changed
                }

                // The type of the assignment expression should be the type of the value
                changed |= self.ty.sync(&mut value.ty)?;

                Ok(changed)
            }

            ExprKind::Conditional(Conditional {
                ref mut cond,
                ref mut on_true,
                ref mut on_false,
            }) => {
                let mut changed = false;
                changed |= cond.ty.push_complete(Type::Scalar(ScalarKind::Bool))?;
                changed |= self.ty.sync(&mut on_true.ty)?;
                changed |= self.ty.sync(&mut on_false.ty)?;
                Ok(changed)
            }
            ExprKind::Loop(Loop { .. }) => self.ty.push_complete(Type::Unit),
            ExprKind::Function(Function {
                ref mut params,
                ref mut body,
                ..
            }) => {
                let mut changed = false;

                // First, assume a base type for the function
                let base_type =
                    Type::Function(vec![Type::Unknown; params.len()], Box::new(Type::Unknown));
                changed |= self.ty.push(&base_type)?;

                if let Type::Function(ref mut param_types, ref mut res_type) = self.ty {
                    // Sync parameter types
                    for (param_ty, param_expr) in param_types.iter_mut().zip(params.iter_mut()) {
                        changed |= param_ty.sync(&mut param_expr.ty)?;
                    }

                    // Infer types for each expression in the body
                    for expr in body.iter_mut() {
                        changed |= expr.infer_locally(env)?;
                    }

                    // The type of the function's body is the type of its last expression
                    if let Some(last_expr) = body.last_mut() {
                        changed |= res_type.sync(&mut last_expr.ty)?;
                    } else {
                        // Handle the case where the function body is empty, if necessary
                        // For example, you might default to Type::Unit or handle it some other way
                        *res_type = Box::new(Type::Unit);
                    }

                    Ok(changed)
                } else {
                    bail!("Expected function type for lambda, got {}", &self.ty)
                }
            }
            ExprKind::Call(Call {
                ref ident,
                ref mut args,
            }) => {
                let mut changed = false;
                // self.ty = Type::Unknown;

                // Infer types for each argument
                for arg in args.iter_mut() {
                    changed |= arg.infer_locally(env)?;
                }

                // Look up the function type using the identifier
                if let Some(func_type) = env.get(ident) {
                    if let Type::Function(ref param_types, ref return_type) = func_type {
                        // Ensure the number of arguments matches the number of parameters
                        if param_types.len() != args.len() {
                            bail!(
                                "Function {} expects {} arguments but got {}",
                                ident,
                                param_types.len(),
                                args.len()
                            );
                        }

                        // Ensure each argument's type matches the corresponding parameter type
                        for (arg, param_type) in args.iter().zip(param_types.iter()) {
                            if arg.ty != *param_type {
                                bail!(
                                    "Expected type {} for argument but got {}",
                                    param_type,
                                    arg.ty
                                );
                            }
                        }

                        // Set the type of the call expression to the return type of the function
                        // changed |= self.ty.push(&**return_type)?;
                        println!("Function Type: {:?}", func_type);
                        println!("Expected Return Type: {:?}", return_type);
                        println!("Call Expression Type Before Assignment: {:?}", self.ty);
                        self.ty = (**return_type).clone();
                        println!("Call Expression Type After Assignment: {:?}", self.ty);
                        changed = true;
                        Ok(changed)
                    } else {
                        bail!(
                            "Expected a function type for {} but got {}",
                            ident,
                            func_type
                        );
                    }
                } else {
                    bail!("Function {} is not defined", ident);
                }
            }
            ExprKind::Block(Block { ref mut exprs }) => {
                let mut changed = false;
                for expr in exprs.iter_mut() {
                    changed |= expr.infer_locally(env)?;
                }
                if let Some(last_expr) = exprs.last() {
                    changed |= self.ty.push(&last_expr.ty)?;
                } else {
                    self.ty.push_complete(Type::Unit)?;
                }
                Ok(changed)
            }
        }
    }

    /// Infer types for an expression upward.
    ///
    /// This method iterates over each expression in the AST in post-order, operating on the trees
    /// leaves and propagating types up. The method returns whether the type of this expression or
    /// any subexpressions changed, or an error if one occurred.
    fn infer_up(&mut self, env: &mut TypeMap) -> Result<bool> {
        let mut changed = false;
        // Remember the old bindings so they can be restored.
        let mut old_bindings: Vec<Binding> = Vec::new();
        // Bindings for the Function variant
        if let ExprKind::Function(Function { ref mut params, .. }) = &mut self.kind {
            for p in params {
                let previous = env.insert(p.ident.clone(), p.ty.clone());
                old_bindings.push((p.ident.clone(), previous));
            }
        }

        match self.kind {
            ExprKind::Identifier(_) => (),
            ExprKind::Literal(_) => (),
            ExprKind::UnaryExpr(UnaryExpr { ref mut child, .. }) => {
                changed |= child.infer_up(env)?;
            }
            ExprKind::BinaryExpr(BinaryExpr {
                ref mut lhs,
                ref mut rhs,
                ..
            }) => {
                changed |= lhs.infer_up(env)?;
                changed |= rhs.infer_up(env)?;
            }
            ExprKind::Return(Return { ref mut value }) => {
                changed |= value.infer_up(env)?;
            }
            ExprKind::Assignment(Assignment {
                ref mut ident,
                ref mut value,
            }) => {
                changed |= value.infer_up(env)?;

                // Add the identifier to the environment if not present.
                if !env.contains_key(ident) {
                    env.insert(ident.clone(), value.ty.clone());
                }

                changed |= self.ty.sync(&mut value.ty)?;
            }

            ExprKind::Function(Function {
                ref mut ident,
                ref mut params,
                ref mut body,
                ..
            }) => {
                env.insert(ident.clone(), self.ty.clone());
                for p in params {
                    let previous = env.insert(p.ident.clone(), p.ty.clone());
                    old_bindings.push((p.ident.clone(), previous));
                }
                for expr in body.iter_mut() {
                    changed |= expr.infer_up(env)?;
                }
            }
            ExprKind::Block(Block { ref mut exprs }) => {
                for expr in exprs.iter_mut() {
                    changed |= expr.infer_up(env)?;
                }
            }
            ExprKind::Loop(Loop { .. }) => {}
            ExprKind::Conditional(Conditional {
                ref mut cond,
                ref mut on_true,
                ref mut on_false,
            }) => {
                changed |= cond.infer_up(env)?;
                changed |= on_true.infer_up(env)?;
                changed |= on_false.infer_up(env)?;
            }
            ExprKind::Call(Call {
                ref ident,
                ref mut args,
            }) => {
                for arg in args.iter_mut() {
                    changed |= arg.infer_up(env)?;
                }
                if let Some(ty) = env.get(ident) {
                    changed |= self.ty.push(ty)?;
                } else {
                    bail!("Function {} is not defined", ident);
                }
            }
        };

        // Undo symbol bindings.
        for (symbol, opt) in old_bindings {
            match opt {
                Some(old) => env.insert(symbol, old),
                None => env.remove(&symbol),
            };
        }

        // Infer local type.
        changed |= self.infer_locally(env)?;
        Ok(changed)
    }
}

pub fn infer_types(exprs: Vec<Expr>) -> Result<Vec<Expr>> {
    let mut exprs = exprs;

    // Construct the global environment with function types.
    let mut global_env = TypeMap::default();
    for expr in &exprs {
        if let ExprKind::Function(Function { ref ident, .. }) = &expr.kind {
            global_env.insert(ident.clone(), expr.ty.clone());
        }
    }

    for expr in exprs.iter_mut() {
        let mut local_env = global_env.clone();
        expr.infer_types_with_env(&mut local_env)?;
    }
    Ok(exprs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::typed::*;
    use crate::syntax::typed_parser::parse;

    #[test]
    fn test_infer_leaves() {
        let mut exprs = parse("1").unwrap();
        let exprs = infer_types(exprs).unwrap();
        assert_eq!(exprs[0].ty, Type::Scalar(ScalarKind::Int));
        let mut exprs = parse("true").unwrap();
        let exprs = infer_types(exprs).unwrap();
        assert_eq!(exprs[0].ty, Type::Scalar(ScalarKind::Bool));
    }

    #[test]
    fn test_infer_unary() {
        let mut expr = parse("-1").unwrap();
        let expr = infer_types(expr).unwrap();
        assert_eq!(expr[0].ty, Type::Scalar(ScalarKind::Int));
    }

    #[test]
    fn test_infer_binary() {
        let mut expr = parse("1 + 2").unwrap();
        let expr = infer_types(expr).unwrap();
        assert_eq!(expr[0].ty, Type::Scalar(ScalarKind::Int));
    }

    #[test]
    fn test_infer_assignment() {
        let mut expr = parse("x = 1").unwrap();
        let expr = infer_types(expr).unwrap();
        assert_eq!(expr[0].ty, Type::Scalar(ScalarKind::Int));
    }

    #[test]
    fn test_infer_function() {
        let mut expr = parse("def f(x: int) -> int { return x }").unwrap();
        let expr = infer_types(expr).unwrap();
        assert_eq!(
            expr[0].ty,
            Type::Function(
                vec![Type::Scalar(ScalarKind::Int)],
                Box::new(Type::Scalar(ScalarKind::Int))
            )
        );
    }

    // FIXME:
    // #[test]
    // fn test_infer_call() {
    //     let source = "
    //     def f(x: int) -> int { return x }
    //     f(1)
    //     ";
    //     let mut expr = parse(source).unwrap();
    //     // dbg!("expr: {:?}", expr.clone());
    //     let expr = infer_types(expr).unwrap();
    //     // dbg!("expr: {:?}", expr);
    //     assert_eq!(expr[1].ty, Type::Scalar(ScalarKind::Int));
    // }

    #[test]
    fn test_infer_conditional() {
        let mut expr = parse("if (true) { 1 } else { 2 }").unwrap();
        let expr = infer_types(expr).unwrap();
        assert_eq!(expr[0].ty, Type::Scalar(ScalarKind::Int));
    }

    #[test]
    fn test_infer_loop() {
        let mut expr = parse("while (true) { 1 }").unwrap();
        let expr = infer_types(expr).unwrap();
        assert_eq!(expr[0].ty, Type::Unit);
    }

    // #[test]
    // fn test_infer_block() {
    //     let mut expr = parse("{ 1 }").unwrap();
    //     let expr = infer_types(expr).unwrap();
    //     assert_eq!(expr[0].ty, Type::Scalar(ScalarKind::Int));
    // }
}
