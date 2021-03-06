#[macro_use]
extern crate gears_lang;

use gears_lang::compiler::compile_file;
use gears_lang::module::disassemble;
use gears_lang::object::GearsObject;
use gears_lang::vm::execute_function;
use std::sync::Arc;

#[test]
fn assignment() {
    let module = compile_file("tests/files/assignment.gs");

    assert_eq!(
        execute_function(&module.expect("Test failure"), "assignment", vec![]).unwrap(),
        gears_obj!(44)
    );
}

#[test]
fn return_assignment() {
    let module = compile_file("tests/files/assignment.gs");

    assert_eq!(
        execute_function(&module.expect("Test failure"), "return_assignment", vec![]).unwrap(),
        gears_obj!(13)
    );
}

#[test]
fn reassignment() {
    let module = compile_file("tests/files/assignment.gs").expect("Test failure");

    disassemble(&module, "reassign");

    assert_eq!(
        execute_function(&module, "reassign", vec![]).unwrap(),
        gears_obj!(9)
    );
}
