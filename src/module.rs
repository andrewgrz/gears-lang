use errors::GearsError;
use object::GearsObject;
use opcodes::*;
use std::collections::HashMap;

/// Contains a compiled module
#[derive(Debug)]
pub struct Module {
    name: String,
    functions: HashMap<String, Function>,
    consts: Vec<GearsObject>
}

impl Module {
    fn new(name: String) -> Module {
        Module {
            name: name,
            functions: HashMap::new(),
            consts: Vec::new(),
        }
    }

    pub fn get_function(&self, name: &str) -> Result<&Function, GearsError> {
        match self.functions.get(&name.to_string()) {
            Some(v) => Ok(v),
            None => Err(GearsError::FunctionNotFound(name.to_string())),
        }
    }

    pub fn get_const(&self, index: usize) -> &GearsObject {
        &self.consts[index]
    }
    
    fn insert_int(&mut self, number: i64) -> usize {
        let new_value = GearsObject::Int(number);

        for (index, constant) in self.consts.iter().enumerate() {
            if constant == &new_value {
                return index;
            }
        } 

        let result = self.consts.len();
        self.consts.push(new_value);
        return result;
    }
}

pub struct ModuleBuilder {
    module: Module,
    current_fn: Option<Function>,
}

impl ModuleBuilder {
    
    pub fn new(name: String) -> ModuleBuilder {
        ModuleBuilder {
            module: Module::new(name),
            current_fn: None,
        }
    }

    pub fn start_function(&mut self, name: String) {
        self.current_fn = Some(Function::new(name));
    }

    pub fn finish_function(&mut self) {
        let mut push_return = false;

        match self.current_fn.as_ref() {
            Some(cur_fn) => {
                match cur_fn.opcodes.last() {
                    Some(op) => {
                        if op != &RETURN {
                            push_return = true;
                        }
                    },
                    None => {
                        push_return = true;
                    }
                }
            }, 
            None => {}
        }

        if push_return {
            self.opcode(RETURN);
        }

        match self.current_fn.as_mut() {
            Some(cur_fn) => {
                self.module.functions.insert(cur_fn.get_name(), cur_fn.clone());
            }
            None => {},
        }

        self.current_fn = None;
    }

    pub fn build(self) -> Module {
        self.module
    }

    #[inline]
    fn opcode(&mut self, opcode: u8) {
        match self.current_fn.as_mut() {
            Some(cur_fn) => cur_fn.opcodes.push(opcode),
            None => {},
        }
    }

    pub fn load_int(&mut self, number: i64) {
        let index = self.module.insert_int(number);
        
        // TODO: Handle int overflow with new opcode

        self.opcode(LOAD_CONST);
        self.opcode(index as u8);
    }

    pub fn op_add(&mut self) {
        self.opcode(BIN_ADD);
    }
}

/// A compiled function
#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    opcodes: Opcodes,
}

impl Function {
    pub fn new(name: String) -> Function {
        Function {
            name: name,
            opcodes: Opcodes::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_opcodes(&self) -> &Opcodes {
        &self.opcodes
    }
}
