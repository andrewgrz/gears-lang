
use module::{Module, Function};
use object::{GearsResult, GearsObject};
use errors::GearsError;
use opcodes::*;

/// Execute a function contained in a compiled module
pub fn execute_function(module: &Module, function: &str) -> GearsResult {
    let mod_fn = module.get_function(function)?;

    execute(&mod_fn, &module)
}

fn execute(function: &Function, module: &Module) -> GearsResult {
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
            LOAD_CONST => {
                advance!();
                push!((*module.get_const(cur_instr as usize)).clone());
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

        module_builder.start_function("simple_math".to_string());
        module_builder.load_int(3);
        module_builder.load_int(4);
        module_builder.op_add();
        module_builder.load_int(8);
        module_builder.op_add();
        module_builder.finish_function();

        let module = module_builder.build();
        let result = execute_function(&module, "simple_math");
        assert_eq!(result, Ok(GearsObject::Int(15)));
    }
}
