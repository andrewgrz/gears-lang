use errors::{GearsError, InterOpErrorType};
use module::{Function, Module};
use object::{ArcGearsObject, ArcGearsResult, FALSE_OBJ, NONE_OBJ, TRUE_OBJ};
use opcodes::*;
use std::sync::Arc;

/// Execute a function contained in a compiled module
pub fn execute_function(
    module: &Module,
    function: &str,
    args: Vec<ArcGearsObject>,
) -> ArcGearsResult {
    let mod_fn = module.get_function(function)?;
    let num_given_args = args.len();
    let num_fn_args = mod_fn.num_args();

    if num_given_args != num_fn_args {
        if num_given_args < num_fn_args {
            return Err(GearsError::InterOpError {
                error: InterOpErrorType::TooFewArgs,
                message: format!(
                    "Function missing args: {}, expected: {}, received: {}",
                    function, num_fn_args, num_given_args
                ),
            });
        } else {
            return Err(GearsError::InterOpError {
                error: InterOpErrorType::TooManyArgs,
                message: format!(
                    "Function pass extra args: {}, expected: {}, received: {}",
                    function, num_fn_args, num_given_args
                ),
            });
        }
    }

    execute(&mod_fn, &module, args)
}

fn execute(function: &Function, module: &Module, mut args: Vec<ArcGearsObject>) -> ArcGearsResult {
    let opcodes = function.get_opcodes();
    let mut cur_instr: u8;
    let mut ip: usize = 0;
    let mut stack: Vec<ArcGearsObject> = Vec::new();

    macro_rules! pop {
        () => {
            match stack.pop() {
                Some(e) => e.clone(),
                None => {
                    return Err(GearsError::InternalCompilerError(
                        "Unexpected Empty Stack".to_string(),
                    ))
                }
            }
        };
    }

    macro_rules! push {
        ($v:expr) => {
            stack.push($v)
        };
    }

    macro_rules! bin_op {
        ($op:ident) => {{
            let b: ArcGearsObject = pop!();
            let a: ArcGearsObject = pop!();
            push!(Arc::new((*a).$op(&*b)?));
        }};
    }

    macro_rules! unary_op {
        ($op:ident) => {{
            let a: ArcGearsObject = pop!();
            push!(Arc::new((*a).$op()?));
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

        // println!("{:?} {:?}", stack, args);
        // println!("Running: {}", print_code(cur_instr));

        match cur_instr {
            RETURN => return Ok(pop!()),
            BIN_ADD => bin_op!(add),
            BIN_SUB => bin_op!(sub),
            BIN_MUL => bin_op!(mul),
            BIN_DIV => bin_op!(div),

            BIN_EQUAL => bin_op!(equal),
            BIN_NOT_EQUAL => bin_op!(nequal),
            BIN_LESS_THAN => bin_op!(less),
            BIN_LESS_THAN_EQUAL => bin_op!(less_eq),
            BIN_GREATER_THAN => bin_op!(greater),
            BIN_GREATER_THAN_EQUAL => bin_op!(greater_eq),

            LOAD_FAST => {
                advance!();

                match args.get(cur_instr as usize) {
                    Some(e) => push!(e.clone()),
                    None => {
                        return Err(GearsError::InternalCompilerError(format!(
                            "LOAD_FAST failed"
                        )))
                    }
                }
            }
            STORE_FAST => {
                advance!();
                let index = cur_instr as usize;

                if args.len() > index {
                    args[index] = pop!();
                } else {
                    args.insert(index, pop!());
                }
            }
            LOAD_CONST => {
                advance!();
                push!(module.get_const(cur_instr as usize).clone());
            }
            CALL_FUNCTION => {
                advance!();
                let fn_index = cur_instr;
                advance!();

                let mut next_args = Vec::new();

                for _ in 0..cur_instr {
                    next_args.push(pop!());
                }
                next_args.reverse();

                push!(execute(
                    module.get_function_by_index(fn_index as usize)?,
                    module,
                    next_args
                )?);
            }
            LOAD_TRUE => {
                push!(TRUE_OBJ.clone());
            }
            LOAD_FALSE => {
                push!(FALSE_OBJ.clone());
            }
            LOAD_NONE => {
                push!(NONE_OBJ.clone());
            }
            JUMP => {
                advance!();
                ip += cur_instr as usize;
            }
            JUMP_ABSOLUTE => {
                cur_instr = opcodes[ip];
                ip = cur_instr as usize;
            }
            JUMP_IF_FALSE => {
                advance!();
                if !pop!().as_bool() {
                    ip += cur_instr as usize;
                }
            }
            INC_ONE => {
                unary_op!(inc);
            }
            _ => {
                return Err(GearsError::InternalCompilerError(format!(
                    "Unexpected Opcode: {:?}",
                    cur_instr
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use module::ModuleBuilder;
    use object::GearsObject;

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
        let result = execute_function(&module, "simple_math", Vec::new());
        assert_eq!(result, Ok(Arc::new(GearsObject::Int(15))));
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
        let result = execute_function(&module, "simple_math", Vec::new());
        assert_eq!(result, Ok(Arc::new(GearsObject::Int(11))));
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
        let result = execute_function(&module, "simple_math", Vec::new());
        assert_eq!(result, Ok(Arc::new(GearsObject::Int(60))));
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
        let result = execute_function(&module, "simple_math", Vec::new());
        assert_eq!(result, Ok(Arc::new(GearsObject::Int(2))));
    }
}
