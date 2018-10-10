use errors::GearsError;
use object::GearsObject;
use opcodes::*;
use std::collections::HashMap;

/// Contains a compiled module
#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    function_lookup: HashMap<String, usize>,
    functions: Vec<Function>,
    consts: Vec<GearsObject>,
}

impl Module {
    fn new(name: String) -> Module {
        Module {
            name: name,
            function_lookup: HashMap::new(),
            functions: Vec::new(),
            consts: Vec::new(),
        }
    }

    pub fn get_function(&self, name: &str) -> Result<&Function, GearsError> {
        match self.function_lookup.get(&name.to_string()) {
            Some(v) => match self.functions.get(*v) {
                Some(v) => Ok(v),
                None => Err(GearsError::InternalCompilerError(
                    "Function Lookup did not point to a valid function".to_string(),
                )),
            },
            None => Err(GearsError::FunctionNotFound(name.to_string())),
        }
    }

    pub fn get_function_by_index(&self, index: usize) -> Result<&Function, GearsError> {
        match self.functions.get(index) {
            Some(v) => Ok(v),
            None => Err(GearsError::FunctionNotFound(format!("{}", index))),
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

    pub fn start_function(&mut self, name: String, num_args: usize) {
        self.current_fn = Some(Function::new(name, num_args));
    }

    pub fn finish_function(&mut self) {
        let mut push_return = false;

        match self.current_fn.as_ref() {
            Some(cur_fn) => match cur_fn.opcodes.last() {
                Some(op) => {
                    if op != &RETURN {
                        push_return = true;
                    }
                }
                None => {
                    push_return = true;
                }
            },
            None => {}
        }

        if push_return {
            self.opcode(RETURN);
        }

        let index = self.module.functions.len();

        match self.current_fn.as_mut() {
            Some(cur_fn) => {
                self.module.functions.push(cur_fn.clone());

                self.module.function_lookup.insert(cur_fn.get_name(), index);
            }
            None => {}
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
            None => {}
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

    pub fn op_sub(&mut self) {
        self.opcode(BIN_SUB);
    }

    pub fn op_mul(&mut self) {
        self.opcode(BIN_MUL);
    }

    pub fn op_div(&mut self) {
        self.opcode(BIN_DIV);
    }

    pub fn store_fast(&mut self, index: u8) {
        self.opcode(STORE_FAST);
        self.opcode(index);
    }

    pub fn load_fast(&mut self, index: u8) {
        self.opcode(LOAD_FAST);
        self.opcode(index);
    }

    pub fn call_fn(&mut self, index: u8, arg_count: u8) {
        self.opcode(CALL_FUNCTION);
        self.opcode(index);
        self.opcode(arg_count);
    }
}

/// A compiled function
#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    num_args: usize,
    opcodes: Opcodes,
}

impl Function {
    pub fn new(name: String, num_args: usize) -> Function {
        Function {
            name: name,
            opcodes: Opcodes::new(),
            num_args: num_args,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_opcodes(&self) -> &Opcodes {
        &self.opcodes
    }

    pub fn num_args(&self) -> usize {
        self.num_args
    }
}
