
extern crate gears_lang;
use gears_lang::parser;

const FULL_GRAMMAR_EXAMPLE: &str = r#"
def simple_math(a, b) {
    let a = 4 - 3;
    let b = a * 9;
    12 * (a + b)
}
"#;

#[test]
fn test_grammar() {
    let result = parser::ModuleParser::new().parse(FULL_GRAMMAR_EXAMPLE);

    match result {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false, "Grammar Error. Please review output.")
        }
        _ => {}
    }
}
