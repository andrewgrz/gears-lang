extern crate gears_lang;
extern crate lalrpop_util;

use gears_lang::parser;
use lalrpop_util::ParseError;

const FULL_GRAMMAR_EXAMPLE: &str = r#"
def exampl(a, b) {
    sub(6 - 2);
    let a = 4 - 3;
    let b = a * 9;
    let c = sub(a - b);
    12 * add(a + 4 - add_two(5), b) + c;

    let d = false;
    let e = true;
}
"#;

#[test]
fn test_grammar() {
    let result = parser::ModuleParser::new().parse(FULL_GRAMMAR_EXAMPLE);

    match result {
        Err(e) => {
            match e {
                ParseError::UnrecognizedToken { token, expected } => {
                    match token {
                        Some(tok) => {
                            println!("At Character: {} Found: '{}'", tok.0, tok.1);
                            let mut cur_line: String = String::new();
                            let mut should_print = false;
                            let mut offset = String::new();

                            for (index, character) in FULL_GRAMMAR_EXAMPLE.chars().enumerate() {
                                if character == '\n' || index == 0 {
                                    if should_print {
                                        println!("{}", cur_line);
                                        println!("{}^", offset);
                                        break;
                                    } else {
                                        cur_line = String::new();
                                    }
                                }

                                cur_line.push(character);

                                if index == tok.0 {
                                    should_print = true;
                                    for _ in 0..(cur_line.len() - 2) {
                                        offset.push(' ');
                                    }
                                }
                            }
                        }
                        None => {
                            println!("Unexpected EOF token");
                        }
                    };
                    println!("Expected: ");
                    for exp_token in expected {
                        print!("{}, ", exp_token);
                    }
                    print!("{}", '\n');
                }
                _ => println!("Unexpected Error: {:?}", e),
            }
            panic!("Grammar Error. Please review output.")
        }
        _ => {}
    }
}
