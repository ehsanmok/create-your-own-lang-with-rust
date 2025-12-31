//! Type System for Thirdlang
//!
//! Extends Secondlang's type system with class types.
//! Classes are nominal types - two classes with the same fields are different types.

use std::collections::HashMap;
use std::fmt;

// ANCHOR: type_enum
/// Types in our language
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// Integer type (64-bit signed)
    Int,
    /// Boolean type
    Bool,
    /// Class type (by name)
    Class(String),
    /// Function type: (param_types) -> return_type
    Function { params: Vec<Type>, ret: Box<Type> },
    /// Method type: (self_type, param_types) -> return_type
    Method {
        class: String,
        params: Vec<Type>,
        ret: Box<Type>,
    },
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
            Type::Class(_) => true,
            Type::Unknown => false,
            Type::Function { params, ret } | Type::Method { params, ret, .. } => {
                params.iter().all(|t| t.is_resolved()) && ret.is_resolved()
            }
        }
    }

    /// Check if this is a class type
    pub fn is_class(&self) -> bool {
        matches!(self, Type::Class(_))
    }

    /// Get class name if this is a class type
    pub fn class_name(&self) -> Option<&str> {
        match self {
            Type::Class(name) => Some(name),
            _ => None,
        }
    }

    // ANCHOR: unify
    /// Try to unify this type with another type
    /// Returns the unified type if successful, or an error message
    pub fn unify(&self, other: &Type) -> Result<Type, String> {
        match (self, other) {
            // Same primitive types unify
            (Type::Int, Type::Int) => Ok(Type::Int),
            (Type::Bool, Type::Bool) => Ok(Type::Bool),
            (Type::Unit, Type::Unit) => Ok(Type::Unit),

            // Same class types unify (nominal typing)
            (Type::Class(a), Type::Class(b)) if a == b => Ok(Type::Class(a.clone())),

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
            Type::Class(name) => write!(f, "{}", name),
            Type::Function { params, ret } => {
                let params_str: Vec<_> = params.iter().map(|t| t.to_string()).collect();
                write!(f, "({}) -> {}", params_str.join(", "), ret)
            }
            Type::Method { class, params, ret } => {
                let params_str: Vec<_> = params.iter().map(|t| t.to_string()).collect();
                write!(f, "{}::({}) -> {}", class, params_str.join(", "), ret)
            }
        }
    }
}

// ANCHOR: class_info
/// Information about a class definition
#[derive(Debug, Clone)]
pub struct ClassInfo {
    /// Class name
    pub name: String,
    /// Fields: name -> type
    pub fields: HashMap<String, Type>,
    /// Field order (for memory layout)
    pub field_order: Vec<String>,
    /// Methods: name -> (param_types, return_type)
    pub methods: HashMap<String, MethodInfo>,
    /// Whether the class has a destructor
    pub has_destructor: bool,
}

/// Information about a method
#[derive(Debug, Clone)]
pub struct MethodInfo {
    /// Method name
    pub name: String,
    /// Parameter types (excluding self)
    pub params: Vec<(String, Type)>,
    /// Return type
    pub return_type: Type,
    /// Is this the constructor?
    pub is_constructor: bool,
    /// Is this the destructor?
    pub is_destructor: bool,
}
// ANCHOR_END: class_info

impl ClassInfo {
    pub fn new(name: String) -> Self {
        ClassInfo {
            name,
            fields: HashMap::new(),
            field_order: Vec::new(),
            methods: HashMap::new(),
            has_destructor: false,
        }
    }

    /// Add a field to the class
    pub fn add_field(&mut self, name: String, ty: Type) {
        self.fields.insert(name.clone(), ty);
        self.field_order.push(name);
    }

    /// Add a method to the class
    pub fn add_method(&mut self, info: MethodInfo) {
        if info.is_destructor {
            self.has_destructor = true;
        }
        self.methods.insert(info.name.clone(), info);
    }

    /// Get field type by name
    pub fn get_field(&self, name: &str) -> Option<&Type> {
        self.fields.get(name)
    }

    /// Get method info by name
    pub fn get_method(&self, name: &str) -> Option<&MethodInfo> {
        self.methods.get(name)
    }

    /// Get field index for memory layout
    pub fn field_index(&self, name: &str) -> Option<usize> {
        self.field_order.iter().position(|n| n == name)
    }

    /// Calculate object size (number of i64 fields)
    pub fn size(&self) -> usize {
        self.fields.len()
    }
}

/// Class registry - maps class names to their definitions
pub type ClassRegistry = HashMap<String, ClassInfo>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unify_same_types() {
        assert_eq!(Type::Int.unify(&Type::Int).unwrap(), Type::Int);
        assert_eq!(Type::Bool.unify(&Type::Bool).unwrap(), Type::Bool);
    }

    #[test]
    fn test_unify_class_types() {
        let point = Type::Class("Point".to_string());
        assert_eq!(point.unify(&point).unwrap(), point);

        let vec = Type::Class("Vec".to_string());
        assert!(point.unify(&vec).is_err());
    }

    #[test]
    fn test_unify_unknown() {
        assert_eq!(Type::Unknown.unify(&Type::Int).unwrap(), Type::Int);
        assert_eq!(Type::Int.unify(&Type::Unknown).unwrap(), Type::Int);
    }

    #[test]
    fn test_class_info() {
        let mut class = ClassInfo::new("Point".to_string());
        class.add_field("x".to_string(), Type::Int);
        class.add_field("y".to_string(), Type::Int);

        assert_eq!(class.size(), 2);
        assert_eq!(class.field_index("x"), Some(0));
        assert_eq!(class.field_index("y"), Some(1));
    }
}
