use errors::GearsError;
use module::Module;
use parser;
use std::fs::File;
use std::io::prelude::*;

/// Compile a gears file to a module
pub fn compile_file(filename: &str) -> Result<Module, GearsError> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    compile_str(&contents, filename)
}

/// Compile a String to
pub fn compile_str(string: &str, name: &str) -> Result<Module, GearsError> {
    let module = Module::new(String::from(name));
    let module_ast = parser::ModuleParser::new().parse(string);

    Ok(module)
}
