
use module::{Module, Function};
use object::{GearsResult, GearsObject};
use errors::{GearsError, InterOpErrorType};
use opcodes::*;

/// Execute a function contained in a compiled module
pub fn execute_function(module: &Module, function: &str, args: Vec<GearsObject>) -> GearsResult {
    let mod_fn = module.get_function(function)?;
    let num_given_args = args.len();
    let num_fn_args = mod_fn.num_args();

    if num_given_args != num_fn_args {
        if num_given_args < num_fn_args {
            return Err(GearsError::InterOpError{ error: InterOpErrorType::TooFewArgs, message: format!("Function missing args: {}, expected: {}, received: {}", function, num_fn_args, num_given_args)});
        } else {
            return Err(GearsError::InterOpError{ error: InterOpErrorType::TooManyArgs, message: format!("Function pass extra args: {}, expected: {}, received: {}", function, num_fn_args, num_given_args)});
        }   
    }

    execute(&mod_fn, &module, args)
}

fn execute(function: &Function, module: &Module, mut args: Vec<GearsObject>) -> GearsResult {
    let opcodes = function.get_opcodes();
    let mut cur_instr: u8;
    let mut ip: usize = 0;
    let mut stack: Vec<GearsObject> = Vec::new();

    macro_rules! pop {
        () => {
            match stack.pop() {
                Some(e) => e,
                None => return Err(GearsError::InternalCompilerError("Unexpected Empty Stack".to_string())),
            }
        }
    }

    macro_rules! push {
        ($v:expr) => {
            stack.push($v)
        };
    }

    macro_rules! bin_op {
        ($op:ident) => {{
            let b = pop!();
            let a = pop!();
            push!(a.$op(b)?);
        }};
    }

    macro_rules! advance {
        () => {{
            cur_instr = opcodes[ip];
            ip += 1;
        }};
    }

    loop {
        advance!();

        match cur_instr {
            RETURN => return Ok(pop!()),
            BIN_ADD => bin_op!(add),
            BIN_SUB => bin_op!(sub),
            BIN_MUL => bin_op!(mul),
            BIN_DIV => bin_op!(div),
            LOAD_FAST => {
                advance!();

                match args.get(cur_instr as usize) {
                    Some(e) => push!(e.clone()),
                    None => return Err(GearsError::InternalCompilerError(format!("LOAD_FAST failed")))
                }
            }
            STORE_FAST => {
                advance!();
                args.insert(cur_instr as usize, pop!());
            }
            LOAD_CONST => {
                advance!();
                push!((*module.get_const(cur_instr as usize)).clone());
            },
            CALL_FUNCTION => {
                advance!();
                let fn_index = cur_instr;
                advance!();

                let mut next_args = Vec::new();

                for _ in 0..cur_instr {
                    next_args.push(pop!());
                }
                next_args.reverse();

                push!(execute(module.get_function_by_index(fn_index as usize)?, module, next_args)?);
            },
            LOAD_TRUE => {
                push!(GearsObject::Bool(true));
            },
            LOAD_FALSE => {
                push!(GearsObject::Bool(false));
            },
            _ => return Err(GearsError::InternalCompilerError(format!("Unexpected Opcode: {:?}", cur_instr))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use module::{ModuleBuilder};

    #[test]
    fn test_addition() {
        let mut module_builder = ModuleBuilder::new("Test".to_string());

        module_builder.start_function("simple_math".to_string(), 0);
        module_builder.load_int(3);
        module_builder.load_int(4);
        module_builder.op_add();
        module_builder.load_int(8);
        module_builder.op_add();
        module_builder.finish_function();

        let module = module_builder.build();
        let result = execute_function(&module, "simple_math", Vec::new(),);
        assert_eq!(result, Ok(GearsObject::Int(15)));
    }

    #[test]
    fn test_subtraction() {
        let mut module_builder = ModuleBuilder::new("Test".to_string());

        module_builder.start_function("simple_math".to_string(), 0);
        module_builder.load_int(20);
        module_builder.load_int(4);
        module_builder.op_sub();
        module_builder.load_int(5);
        module_builder.op_sub();
        module_builder.finish_function();

        let module = module_builder.build();
        let result = execute_function(&module, "simple_math", Vec::new(),);
        assert_eq!(result, Ok(GearsObject::Int(11)));
    }

    #[test]
    fn test_mul() {
        let mut module_builder = ModuleBuilder::new("Test".to_string());

        module_builder.start_function("simple_math".to_string(), 0);
        module_builder.load_int(3);
        module_builder.load_int(4);
        module_builder.op_mul();
        module_builder.load_int(5);
        module_builder.op_mul();
        module_builder.finish_function();

        let module = module_builder.build();
        let result = execute_function(&module, "simple_math", Vec::new(),);
        assert_eq!(result, Ok(GearsObject::Int(60)));
    }

    #[test]
    fn test_div() {
        let mut module_builder = ModuleBuilder::new("Test".to_string());

        module_builder.start_function("simple_math".to_string(), 0);
        module_builder.load_int(50);
        module_builder.load_int(5);
        module_builder.op_div();
        module_builder.load_int(5);
        module_builder.op_div();
        module_builder.finish_function();

        let module = module_builder.build();
        let result = execute_function(&module, "simple_math", Vec::new(),);
        assert_eq!(result, Ok(GearsObject::Int(2)));
    } 
}
