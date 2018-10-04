use ast::ModStmtAst;
use errors::GearsError;
use module::{Module, ModuleBuilder};
use parser;
use std::fs::File;
use std::io::prelude::*;
use symbol::SymbolTable;

/// Compile a gears file to a module
pub fn compile_file(filename: &str) -> Result<Module, GearsError> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    compile_str(&contents, filename)
}

/// Compile a String to module
pub fn compile_str(string: &str, name: &str) -> Result<Module, GearsError> {
    compile_ast(parser::ModuleParser::new().parse(string)?, name)
}

/// Compiles AST to Module and Bytecode
fn compile_ast(ast: Vec<Box<ModStmtAst>>, name: &str) -> Result<Module, GearsError> {
    let mut module_builder = ModuleBuilder::new(String::from(name));
    let mut symbol_table = SymbolTable::new_global();

    // Add all the top level functions to the scope
    for mod_stmt in ast {
        match *mod_stmt {
            ModStmtAst::FunctionDef { name, .. } => {
                symbol_table.def_fn(name);
            }
        }
    }

    Ok(module_builder.build())
}
