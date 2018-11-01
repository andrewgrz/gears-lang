extern crate gears_lang;
#[macro_use]
extern crate cached;
#[macro_use]
extern crate lazy_static;

use gears_lang::compiler::compile_file;
use gears_lang::module::{disassemble, Module};
use gears_lang::object::GearsObject;
use gears_lang::vm::execute_function;

cached!{
    FIB;
    fn setup() -> Module = {
        compile_file("tests/files/comparison.gs").expect("Test failure")
    }
}

#[test]
fn test_compare() {
    let test_set = vec![
        // a, b, <,   <=,   >,    >=,     ==,    !=
        (3, 4, true, true, false, false, false, true),
        (4, 4, false, true, false, true, true, false),
        (5, 4, false, false, true, true, false, true),
    ];

    for row in test_set {
        let a = GearsObject::from(row.0);
        let b = GearsObject::from(row.1);

        assert_eq!(
            execute_function(&setup(), "compare_less", vec![a.clone(), b.clone()]).unwrap(),
            GearsObject::Bool(row.2),
            "Compare Failed for: {} < {}",
            row.0,
            row.1
        );

        assert_eq!(
            execute_function(&setup(), "compare_less_eq", vec![a.clone(), b.clone()]).unwrap(),
            GearsObject::Bool(row.3),
            "Compare Failed for: {} <= {}",
            row.0,
            row.1
        );

        assert_eq!(
            execute_function(&setup(), "compare_greater", vec![a.clone(), b.clone()]).unwrap(),
            GearsObject::Bool(row.4),
            "Compare Failed for: {} > {}",
            row.0,
            row.1
        );

        assert_eq!(
            execute_function(&setup(), "compare_greater_eq", vec![a.clone(), b.clone()]).unwrap(),
            GearsObject::Bool(row.5),
            "Compare Failed for: {} >= {}",
            row.0,
            row.1
        );

        assert_eq!(
            execute_function(&setup(), "compare_eq", vec![a.clone(), b.clone()]).unwrap(),
            GearsObject::Bool(row.6),
            "Compare Failed for: {} == {}",
            row.0,
            row.1
        );

        assert_eq!(
            execute_function(&setup(), "compare_not_eq", vec![a.clone(), b.clone()]).unwrap(),
            GearsObject::Bool(row.7),
            "Compare Failed for: {} != {}",
            row.0,
            row.1
        );
    }
}
