use opcodes::Opcodes;
use std::collections::HashMap;
use errors::GearsError;

/// Contains a compiled module
pub struct Module {
    name: String,
    functions: HashMap<String, Function>,
}

impl Module {
    pub fn new(name: String) -> Module {
        Module {
            name: name,
            functions: HashMap::new(),
        }
    }

    pub fn get_function(&self, name: &str) -> Result<&Function, GearsError> {
        match self.functions.get(&name.to_string()) {
            Some(v) => Ok(v),
            None => Err(GearsError::FunctionNotFound(name.to_string()))
        }
    }
}

/// A compiled function
pub struct Function {
    name: String,
    opcodes: Opcodes,
}

impl Function {
    pub fn get_opcodes(&self) -> &Opcodes {
        &self.opcodes
    }
}
