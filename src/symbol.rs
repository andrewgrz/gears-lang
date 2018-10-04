
use std::collections::HashMap;

enum Symbol {
    Function,
    Variable,
}

pub struct SymbolTable<'a> {
    parent: Option<&'a SymbolTable<'a>>,
    symbols: HashMap<String, Symbol>,
}

impl <'a> SymbolTable<'a> {
    pub fn new_global() -> SymbolTable<'a> {
        SymbolTable {
            parent: None,
            symbols: HashMap::new(),
        }
    }
 
    pub fn push(&'a self) -> SymbolTable<'a> {
        SymbolTable {
            symbols: HashMap::new(),
            parent: Some(&self),
        }
    }

    pub fn pop(self) -> &'a SymbolTable<'a> {
        match self.parent {
            Some(e) => e,
            None => panic!("Unable to pop the global scope"),
        }
    }
}

