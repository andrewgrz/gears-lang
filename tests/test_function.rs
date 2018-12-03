#[macro_use]
extern crate gears_lang;
#[macro_use]
extern crate cached;
#[macro_use]
extern crate lazy_static;

use gears_lang::compiler::compile_file;
use gears_lang::errors::*;
use gears_lang::module::Module;
use gears_lang::object::{GearsObject, NONE_OBJ};
use gears_lang::vm::execute_function;
use std::sync::Arc;

cached!{
    FIB;
    fn setup() -> Module = {
        compile_file("tests/files/function.gs").expect("Test failure")
    }
}

#[test]
fn function_calling() {
    assert_eq!(
        execute_function(&setup(), "expr_test", vec![]).unwrap(),
        NONE_OBJ.clone()
    );
}

#[test]
fn expr_in_call() {
    assert_eq!(
        execute_function(&setup(), "main_no_args", vec![]).unwrap(),
        gears_obj!(92)
    );
}

#[test]
fn returns_none() {
    assert_eq!(
        execute_function(&setup(), "returns_none", vec![gears_obj!(1), gears_obj!(9)]).unwrap(),
        NONE_OBJ.clone()
    );
}

#[test]
fn pass_args() {
    assert_eq!(
        execute_function(&setup(), "main_args", vec![gears_obj!(1), gears_obj!(9)]).unwrap(),
        gears_obj!(92)
    );
}

#[test]
fn pass_too_few_args() {
    match execute_function(&setup(), "main_args", vec![gears_obj!(1)]) {
        Ok(_) => panic!("Should not have passed"),
        Err(error) => match error {
            GearsError::InterOpError { error, .. } => match error {
                InterOpErrorType::TooFewArgs => {}
                _ => panic!("Wrong error returned"),
            },
            _ => panic!("Wrong error returned"),
        },
    }
}

#[test]
fn pass_too_many_args() {
    match execute_function(
        &setup(),
        "main_args",
        vec![gears_obj!(1), gears_obj!(1), gears_obj!(1)],
    ) {
        Ok(_) => panic!("Should not have passed"),
        Err(error) => match error {
            GearsError::InterOpError { error, .. } => match error {
                InterOpErrorType::TooManyArgs => {}
                _ => panic!("Wrong error returned"),
            },
            _ => panic!("Wrong error returned"),
        },
    }
}
