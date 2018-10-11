extern crate gears_lang;
#[macro_use]
extern crate cached;
#[macro_use]
extern crate lazy_static;

use gears_lang::compiler::compile_file;
use gears_lang::module::Module;
use gears_lang::object::GearsObject;
use gears_lang::vm::execute_function;

cached!{
    FIB;
    fn setup() -> Module = {
        compile_file("tests/files/branching.gs").expect("Test failure")
    }
}

#[test]
fn test_bool_return() {
    assert_eq!(
        execute_function(&setup(), "test_true", vec![]).unwrap(),
        GearsObject::Bool(true)
    );

    assert_eq!(
        execute_function(&setup(), "test_false", vec![]).unwrap(),
        GearsObject::Bool(false)
    );
}
