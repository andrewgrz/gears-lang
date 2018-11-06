
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SymbolType {
    Function,
    Variable,
}
#[derive(Debug, Clone)]
pub struct Symbol {
    sym_type: SymbolType,
    index: u8,
}

impl Symbol {
    fn new_fn(index: u8) -> Symbol {
        Symbol {
            sym_type: SymbolType::Function,
            index: index
        }
    }

    fn new_var(index: u8) -> Symbol {
        Symbol {
            sym_type: SymbolType::Variable,
            index: index
        }
    }

    pub fn get_type(&self) -> &SymbolType {
        &self.sym_type
    }

    pub fn get_index(&self) -> &u8 {
        &self.index
    }
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

    pub fn is_global(&self) -> bool {
        self.parent.is_none()
    }

    fn get_next_index(&self) -> u8 {
        if self.parent.is_none() || self.parent.unwrap().is_global() {
            self.symbols.len() as u8
        } else {
            self.parent.unwrap().get_next_index() + self.symbols.len() as u8
        }
    }

    pub fn def_fn(&mut self, name: String) -> u8 {
        let index = self.get_next_index();
        self.symbols.insert(name, Symbol::new_fn(index));
        index
    }

    pub fn def_variable(&mut self, name: String) -> u8 {
        let index = self.get_next_index();
        self.symbols.insert(name, Symbol::new_var(index));
        index
    }

    /// Resolves a variable
    /// 
    /// The second value is true if we resolved in the global scope
    pub fn resolve(&self, name: &String) -> (Option<&Symbol>, bool) {
        match self.symbols.get(name) {
            Some(e) => (Some(e), self.parent.is_none()),
            None => {
                match self.parent {
                    Some(p) => p.resolve(name),
                    None => (None, self.parent.is_none())
                }
            }
        }
    }
}
