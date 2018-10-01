
extern crate gears_lang;

use std::fs::File;
use std::io::prelude::*;
use gears_lang::compiler::compile_file;
use gears_lang::vm::execute_module;
use gears_lang::object::GearsObject;

#[test]
fn math() {
    let module = compile_file("tests/files/math.gs");

    if module.is_err() {
        panic!(module)
    }

    // The result will be 12
    assert_eq!(execute_module(module.expect("Test")).unwrap(), GearsObject::Int(4 + 3 * 5 - 42 / 6));
}
