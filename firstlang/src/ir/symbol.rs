use std::cmp::max;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::ast::typed as ast;
use crate::ast::typed::Type;

/// Name used for placeholder expressions.
const PLACEHOLDER_NAME: &str = "#placeholder";

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol {
    name: Rc<String>,
    id: i32,
}

impl Symbol {
    pub fn new<T: Into<String>>(name: T, id: i32) -> Self {
        Self {
            name: Rc::new(name.into()),
            id,
        }
    }

    pub fn placeholder() -> Symbol {
        Symbol::new(PLACEHOLDER_NAME, 0)
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if self.id == 0 {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{}__{}", self.name, self.id)
        }
    }
}

impl From<usize> for Symbol {
    fn from(item: usize) -> Self {
        // Create a Symbol from the given usize.
        // This is just a generic example; you might need to adjust it according to your Symbol implementation.
        Symbol::new(format!("F{}", item), item as i32)
    }
}

/// Utility struct that can track and generate unique IDs and symbols for use in an expression.
/// Each SymbolGenerator tracks the maximum ID used for every symbol name, and can be used to
/// create new symbols with the same name but a unique ID.
#[derive(Debug, Clone)]
pub struct SymbolGenerator {
    id_map: HashMap<String, i32>,
}

impl SymbolGenerator {
    /// Initialize a SymbolGenerator with no existing symbols.
    pub fn new() -> Self {
        SymbolGenerator {
            id_map: HashMap::default(),
        }
    }

    pub fn new_symbol(&mut self, name: &str) -> Symbol {
        let id = self.id_map.entry(name.to_owned()).or_insert(-1);
        *id += 1;
        Symbol::new(name, *id)
    }
}
