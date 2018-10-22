extern crate gears_lang;
extern crate lalrpop_util;

use lalrpop_util::ParseError;

const FULL_GRAMMAR_EXAMPLE: &str = r#"
# Top Level Comment
def example(a, b) {
    sub(6 - 2);   # This is a function that is not bound
    # This is a new binding
    let a = 4 - 3;
    let b = a * 9;
    let c = sub(a - b);
    12 * add(a + 4 - add_two(5), b) + c;

    let d = false;
    let e = true;
}

def branching() {
    if false {
        let other_variable = 4;
    } else {
        let other_variable = 5;
    };

    while true {
        let x = 3;
    };
    
    if true {
        let a = 4 - 3;
    } 
}
"#;

#[test]
fn test_grammar() {
    let result = gears_lang::parse_str(FULL_GRAMMAR_EXAMPLE);

    match result {
        Err(e) => {
            match e {
                ParseError::UnrecognizedToken { token, expected } => {
                    match token {
                        Some(tok) => {
                            println!("At Character: {} Found: '{:?}'", tok.0, tok.1);

                            for (line_num, line) in FULL_GRAMMAR_EXAMPLE.lines().enumerate() {
                                if line_num + 1 == tok.0.line() {
                                    println!("{}", line);
                                    for _ in 0 .. (tok.0.column() - 1) {
                                        print!(" ")
                                    }
                                    for _ in 0 .. (tok.2.column() - tok.0.column() ) {
                                        print!("-")
                                    }
                                }
                            }
                        }
                        None => {
                            println!("Unexpected EOF token");
                        }
                    };
                    println!("\nExpected: ");
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
