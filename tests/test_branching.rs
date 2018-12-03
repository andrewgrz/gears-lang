#[macro_use]
extern crate gears_lang;
#[macro_use]
extern crate cached;
#[macro_use]
extern crate lazy_static;

use gears_lang::compiler::compile_file;
use gears_lang::module::{disassemble, Module};
use gears_lang::object::{GearsObject, FALSE_OBJ, NONE_OBJ, TRUE_OBJ};
use gears_lang::vm::execute_function;
use std::sync::Arc;

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
        TRUE_OBJ.clone()
    );

    assert_eq!(
        execute_function(&setup(), "test_false", vec![]).unwrap(),
        FALSE_OBJ.clone()
    );
}

#[test]
fn test_simple_branch_true() {
    assert_eq!(
        execute_function(&setup(), "simple_branch", vec![TRUE_OBJ.clone()]).unwrap(),
        gears_obj!(5)
    );
}

#[test]
fn test_simple_branch_false() {
    assert_eq!(
        execute_function(&setup(), "simple_branch", vec![FALSE_OBJ.clone()]).unwrap(),
        gears_obj!(4)
    );
}

#[test]
fn test_five_or_none_true() {
    assert_eq!(
        execute_function(&setup(), "five_or_none", vec![TRUE_OBJ.clone()]).unwrap(),
        gears_obj!(5)
    );
}

#[test]
fn test_five_or_none_false() {
    assert_eq!(
        execute_function(&setup(), "five_or_none", vec![NONE_OBJ.clone()]).unwrap(),
        NONE_OBJ.clone()
    );
}

#[test]
fn test_simple_while_loop() {
    disassemble(&setup(), "while_loop");
    assert_eq!(
        execute_function(&setup(), "while_loop", vec![]).unwrap(),
        gears_obj!(5)
    );
}

#[test]
fn test_simple_for_loop() {
    disassemble(&setup(), "for_loop");
    assert_eq!(
        execute_function(&setup(), "for_loop", vec![]).unwrap(),
        gears_obj!(19)
    );
}
