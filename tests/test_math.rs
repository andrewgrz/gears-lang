
extern crate gears;

use std::fs::File;
use std::io::prelude::*;
use gears::compiler::compile_file;
use gears::vm::execute_module;
use gears::object::GearsObject;

#[test]
fn math() {
    let module = compile_file("tests/files/math.gs");

    if module.is_err() {
        panic!(module)
    }

    // The result will be 12
    assert_eq!(execute_module(module.expect("Test")).unwrap(), GearsObject::Int(4 + 3 * 5 - 42 / 6));
}
