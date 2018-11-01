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
        self.opcode(RETURN);
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

    #[inline]
    fn set_opcode_at(&mut self, index: usize, opcode: u8) {
        match self.current_fn.as_mut() {
            Some(cur_fn) => cur_fn.opcodes[index] = opcode,
            None => {}
        }
    }

    /// Get the index of the last opcode in the fn
    #[inline]
    fn last_index(&self) -> usize {
        let index = match self.current_fn.as_ref() {
            Some(cur_fn) => cur_fn.opcodes.len(),
            None => 1,
        };
        index - 1
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

    pub fn op_eqeq(&mut self) {
        self.opcode(BIN_EQUAL);
    }

    pub fn op_not_eq(&mut self) {
        self.opcode(BIN_NOT_EQUAL);
    }

    pub fn op_less(&mut self) {
        self.opcode(BIN_LESS_THAN);
    }

    pub fn op_less_eq(&mut self) {
        self.opcode(BIN_LESS_THAN_EQUAL);
    }

    pub fn op_greater(&mut self) {
        self.opcode(BIN_GREATER_THAN);
    }

    pub fn op_greater_eq(&mut self) {
        self.opcode(BIN_GREATER_THAN_EQUAL);
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

    pub fn load_none(&mut self) {
        self.opcode(LOAD_NONE);
    }

    pub fn load_bool(&mut self, b: &bool) {
        if *b {
            self.opcode(LOAD_TRUE);
        } else {
            self.opcode(LOAD_FALSE);
        }
    }

    pub fn start_loop_check(&mut self) -> usize {
        self.last_index()
    }

    pub fn end_loop(&mut self, loop_index: usize, jump_index: usize) {
        self.opcode(JUMP_ABSOLUTE);
        self.opcode(loop_index as u8 + 1);
        let cur_index = self.last_index();
        self.set_opcode_at(jump_index, (cur_index - jump_index) as u8);
    }

    pub fn start_jump_if_false(&mut self) -> usize {
        self.opcode(JUMP_IF_FALSE);
        self.opcode(0); // Placeholder
        self.last_index()
    }

    pub fn start_else(&mut self, index: usize) -> usize {
        self.opcode(JUMP);
        self.opcode(0); // Placeholder
        let cur_index = self.last_index();
        self.set_opcode_at(index, (cur_index - index) as u8);
        cur_index
    }

    pub fn end_jump(&mut self, index: usize) {
        let cur_index = self.last_index();
        self.set_opcode_at(index, (cur_index - index) as u8);
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

pub fn disassemble(module: &Module, function: &str) {
    let opcodes = module.get_function(function).unwrap().get_opcodes();

    let mut ip = 0;
    let mut cur_instr: u8;

    macro_rules! advance {
        () => {{
            cur_instr = opcodes[ip];
            ip += 1;
        }};
    }

    macro_rules! print_code {
        ($code:expr, $arg_count:expr) => {{
            print!("{} {}", ip - 1, $code);
            for _ in 0..$arg_count {
                advance!();
                print!(" {}", cur_instr)
            }
            println!();
        }};
    }

    loop {
        advance!();

        match cur_instr {
            RETURN => {
                print_code!("RETURN", 0);
                break;
            }
            CALL_FUNCTION => print_code!("CALL_FUNCTION", 2),
            JUMP => print_code!("JUMP", 1),
            JUMP_ABSOLUTE => print_code!("JUMP_ABSOLUTE", 1),
            JUMP_IF_FALSE => print_code!("JUMP_IF_FALSE", 1),

            // Binary Opcodes
            BIN_ADD => print_code!("BIN_ADD", 0),
            BIN_SUB => print_code!("BIN_SUB", 0),
            BIN_MUL => print_code!("BIN_MUL", 0),
            BIN_DIV => print_code!("BIN_DIV", 0),
            BIN_EQUAL => print_code!("BIN_EQUAL", 0),
            BIN_NOT_EQUAL => print_code!("BIN_NOT_EQUAL", 0),
            BIN_LESS_THAN => print_code!("BIN_LESS_THAN", 0),
            BIN_LESS_THAN_EQUAL => print_code!("BIN_LESS_THAN_EQUAL", 0),
            BIN_GREATER_THAN => print_code!("BIN_GREATER_THAN", 0),
            BIN_GREATER_THAN_EQUAL => print_code!("BIN_GREATER_THAN_EQUAL", 0),

            // Misc Opcodes
            LOAD_CONST => print_code!("LOAD_CONST", 1),

            // Loading and Storing
            LOAD_FAST => print_code!("LOAD_FAST", 1),
            STORE_FAST => print_code!("STORE_FAST", 1),
            LOAD_TRUE => print_code!("LOAD_TRUE", 1),
            LOAD_FALSE => print_code!("LOAD_FALSE", 1),
            LOAD_NONE => print_code!("LOAD_NONE", 1),
            _ => println!("Unexpected opcode!"),
        }
    }
}
