
extern crate gears_lang;

use gears_lang::compiler::compile_file;
use gears_lang::object::GearsObject;
use gears_lang::vm::execute_function;

#[test]
fn assignment() {
    let module = compile_file("tests/files/assignment.gs");

    assert_eq!(
        execute_function(&module.expect("Test failure"), "assignment", vec![]).unwrap(),
        GearsObject::Int(44)
    );
}

#[test]
fn return_assignment() {
    let module = compile_file("tests/files/assignment.gs");

    assert_eq!(
        execute_function(&module.expect("Test failure"), "return_assignment", vec![]).unwrap(),
        GearsObject::Int(13)
    );
}
