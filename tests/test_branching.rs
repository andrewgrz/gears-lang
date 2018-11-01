extern crate gears_lang;
#[macro_use]
extern crate cached;
#[macro_use]
extern crate lazy_static;

use gears_lang::compiler::compile_file;
use gears_lang::module::{Module, disassemble};
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

#[test]
fn test_simple_branch_true() {
    assert_eq!(
        execute_function(&setup(), "simple_branch", vec![GearsObject::Bool(true)]).unwrap(),
        GearsObject::Int(5)
    );
}

#[test]
fn test_simple_branch_false() {
    assert_eq!(
        execute_function(&setup(), "simple_branch", vec![GearsObject::Bool(false)]).unwrap(),
        GearsObject::Int(4)
    );
}

#[test]
fn test_five_or_none_true() {
    assert_eq!(
        execute_function(&setup(), "five_or_none", vec![GearsObject::Bool(true)]).unwrap(),
        GearsObject::Int(5)
    );
}

#[test]
fn test_five_or_none_false() {
    assert_eq!(
        execute_function(&setup(), "five_or_none", vec![GearsObject::None]).unwrap(),
        GearsObject::None
    );
}

#[test]
fn test_simple_while_loop() {
    disassemble(&setup(), "while_loop");
    assert_eq!(
        execute_function(&setup(), "while_loop", vec![]).unwrap(),
        GearsObject::Int(5)
    );
}
