//! Type System for Secondlang
//!
//! This module defines the type system and type inference for Secondlang.
//! Types enable LLVM code generation by providing static type information.

use std::fmt;

// ANCHOR: type_enum
/// Types in our language
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// Integer type (64-bit signed)
    Int,
    /// Boolean type
    Bool,
    /// Function type: (param_types) -> return_type
    Function { params: Vec<Type>, ret: Box<Type> },
    /// Unit type (for statements with no value)
    Unit,
    /// Unknown type (for type inference)
    Unknown,
}
// ANCHOR_END: type_enum

impl Type {
    /// Check if this type is fully resolved (no Unknown types)
    pub fn is_resolved(&self) -> bool {
        match self {
            Type::Int | Type::Bool | Type::Unit => true,
            Type::Unknown => false,
            Type::Function { params, ret } => {
                params.iter().all(|t| t.is_resolved()) && ret.is_resolved()
            }
        }
    }

    // ANCHOR: unify
    /// Try to unify this type with another type
    /// Returns the unified type if successful, or an error message
    pub fn unify(&self, other: &Type) -> Result<Type, String> {
        match (self, other) {
            // Same types unify
            (Type::Int, Type::Int) => Ok(Type::Int),
            (Type::Bool, Type::Bool) => Ok(Type::Bool),
            (Type::Unit, Type::Unit) => Ok(Type::Unit),

            // Unknown can unify with anything
            (Type::Unknown, t) | (t, Type::Unknown) => Ok(t.clone()),

            // Function types must have compatible signatures
            (
                Type::Function {
                    params: p1,
                    ret: r1,
                },
                Type::Function {
                    params: p2,
                    ret: r2,
                },
            ) if p1.len() == p2.len() => {
                let params: Result<Vec<_>, _> =
                    p1.iter().zip(p2.iter()).map(|(a, b)| a.unify(b)).collect();
                let ret = r1.unify(r2)?;
                Ok(Type::Function {
                    params: params?,
                    ret: Box::new(ret),
                })
            }

            // Type mismatch
            _ => Err(format!(
                "Type mismatch: expected {:?}, got {:?}",
                self, other
            )),
        }
    }
    // ANCHOR_END: unify
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
            Type::Unit => write!(f, "()"),
            Type::Unknown => write!(f, "?"),
            Type::Function { params, ret } => {
                let params_str: Vec<_> = params.iter().map(|t| t.to_string()).collect();
                write!(f, "({}) -> {}", params_str.join(", "), ret)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unify_same_types() {
        assert_eq!(Type::Int.unify(&Type::Int).unwrap(), Type::Int);
        assert_eq!(Type::Bool.unify(&Type::Bool).unwrap(), Type::Bool);
    }

    #[test]
    fn test_unify_unknown() {
        assert_eq!(Type::Unknown.unify(&Type::Int).unwrap(), Type::Int);
        assert_eq!(Type::Int.unify(&Type::Unknown).unwrap(), Type::Int);
    }

    #[test]
    fn test_unify_mismatch() {
        assert!(Type::Int.unify(&Type::Bool).is_err());
    }
}
