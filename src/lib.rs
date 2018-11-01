extern crate lalrpop_util;

mod ast;
mod lexer;
mod opcodes;
mod symbol;

pub mod compiler;
pub mod errors;
pub mod module;
pub mod object;
mod parser;
pub mod vm;

pub fn parse_str(
    input: &str,
) -> Result<
    ::std::vec::Vec<Box<ast::ModStmtAst>>,
    lalrpop_util::ParseError<lexer::Span, lexer::Token, lexer::LexicalError>,
> {
    parser::ModuleParser::new().parse(lexer::lex(input))
}
