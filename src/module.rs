use opcodes::Opcodes;
use std::collections::HashMap;

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
}

/// A compiled function
pub struct Function {
    name: String,
    opcodes: Opcodes,
}
