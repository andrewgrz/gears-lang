extern crate gears_lang;
#[macro_use]
extern crate cached;
#[macro_use]
extern crate lazy_static;

use gears_lang::compiler::compile_str;
use gears_lang::errors::*;
use gears_lang::module::Module;
use gears_lang::object::GearsObject;
use gears_lang::vm::execute_function;

#[test]
fn bad_typing() {
    let strings = vec![
        r#"def test() -> int { let a: int = "test" }"#,
        r#"def test() -> int { let a: int = 1 + true }"#,
        r#"def test() -> int { let a: int = false + 2 }"#,
        r#"def test() -> int { let a: int = true }"#,
        r#"def test() -> str { 1 }"#,
        r#"def test() -> bool { 1 }"#,
        r#"def test() -> list { 1 }"#,
    ];

    for (index, string) in strings.iter().enumerate() {
        let res = compile_str(string, &format!("string-{}", index));

        match res {
            Ok(_) => panic!(format!("String passed. {:?}", string)),
            Err(e) => match e {
                GearsError::TypeError(_) => {}
                _ => {
                    println!("{:?}", e);
                    panic!("Not a type error {:?}", string);
                }
            },
        }
    }
}
