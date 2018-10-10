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
        compile_file("tests/files/function.gs").expect("Test failure")
    }
}

#[test]
fn function_calling() {
    assert_eq!(
        execute_function(&setup(), "expr_test", vec![]).unwrap(),
        GearsObject::Int(5)
    );
}

#[test]
fn expr_in_call() {
    assert_eq!(
        execute_function(&setup(), "main_no_args", vec![]).unwrap(),
        GearsObject::Int(92)
    );
}

#[test]
fn pass_args() {
    assert_eq!(
        execute_function(
            &setup(),
            "main_args",
            vec![GearsObject::Int(1), GearsObject::Int(9)]
        ).unwrap(),
        GearsObject::Int(92)
    );
}

#[test]
fn pass_too_few_args() {
    match execute_function(&setup(), "main_args", vec![GearsObject::Int(1)]) {
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
        vec![
            GearsObject::Int(1),
            GearsObject::Int(1),
            GearsObject::Int(1),
        ],
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
