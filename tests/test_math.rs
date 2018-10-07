extern crate gears_lang;

use gears_lang::compiler::compile_file;
use gears_lang::object::GearsObject;
use gears_lang::vm::execute_function;

#[test]
fn math() {
    let module = compile_file("tests/files/math.gs");

    if module.is_err() {
        panic!(module)
    }

    // The result will be 12
    assert_eq!(
        execute_function(&module.expect("Test"), "basic_math", vec![]).unwrap(),
        GearsObject::Int(4 + 3 * 5 - 42 / 6)
    );
}
