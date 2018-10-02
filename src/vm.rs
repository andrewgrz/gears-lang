
use module::{Module, Function};
use object::{GearsResult, GearsObject};
use errors::GearsError;
use opcodes::*;

/// Execute a function contained in a compiled module
pub fn execute_function(module: Module, function: &str) -> GearsResult {
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

    loop {
        cur_instr = opcodes[ip];
        ip += 1;

        match cur_instr {
            RETURN => return Ok(pop!()),
            BIN_ADD => bin_op!(add),
            BIN_SUB => bin_op!(sub),
            _ => return Err(GearsError::InternalCompilerError(format!("Unexpected Opcode: {:?}", cur_instr))),
        }
    }
}
