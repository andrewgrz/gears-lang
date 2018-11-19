extern crate gears_lang;
#[macro_use]
extern crate cached;
#[macro_use]
extern crate lazy_static;

use gears_lang::compiler::compile_file;
use gears_lang::errors::*;
use gears_lang::module::Module;
use gears_lang::object::GearsObject;
use gears_lang::vm::execute_function;

cached!{
    FIB;
    fn setup() -> Module = {
        compile_file("tests/files/strings.gs").expect("Test failure")
    }
}

#[test]
fn return_string() {
    assert_eq!(
        execute_function(&setup(), "simple_string", vec![]).unwrap(),
        GearsObject::from("simple string")
    );
}
