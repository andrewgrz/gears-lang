use ast::*;
use errors::GearsError;
use module::{Module, ModuleBuilder};
use parser;
use lexer;
use std::fs::File;
use std::io::prelude::*;
use symbol::{SymbolTable, SymbolType};

/// Compile a gears file to a module
pub fn compile_file(filename: &str) -> Result<Module, GearsError> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    compile_str(&contents, filename)
}

/// Compile a String to module
pub fn compile_str(string: &str, name: &str) -> Result<Module, GearsError> {
    compile_ast(parser::ModuleParser::new().parse(lexer::lex(string))?, name)
}

/// Compiles AST to Module and Bytecode
fn compile_ast(ast: Vec<Box<ModStmtAst>>, name: &str) -> Result<Module, GearsError> {
    let mut module_builder = ModuleBuilder::new(String::from(name));
    let mut symbol_table = SymbolTable::new_global();

    // Add all the top level functions to the scope before parsing so we can
    // use them during parse as they will resolve
    for ref mod_stmt in &ast {
        match mod_stmt.as_ref() {
            ModStmtAst::FunctionDef { name, .. } => {
                symbol_table.def_fn(name.clone());
            }
        }
    }

    for ref mod_stmt in &ast {
        match mod_stmt.as_ref() {
            ModStmtAst::FunctionDef { name, exprs, args } => {
                module_builder.start_function(name.clone(), args.len());
                let mut local_scope = (&symbol_table).push();

                for arg in args {
                    local_scope.def_variable(arg.name().clone());
                }

                for stmt in exprs {
                    visit_stmt(stmt.as_ref(), &mut local_scope, &mut module_builder)?;
                }

                module_builder.finish_function();
            }
        }
    }

    Ok(module_builder.build())
}

fn visit_stmt(
    stmt: &StmtAst,
    scope: &mut SymbolTable,
    mut module_builder: &mut ModuleBuilder,
) -> Result<(), GearsError> {
    match stmt {
        StmtAst::Expr(e) => visit_expr(e, scope, module_builder)?,
        StmtAst::Assignment { name, expr } => {
            visit_expr(expr, scope, &mut module_builder)?;
            let index = scope.def_variable(name.clone());
            module_builder.store_fast(index);
        }
    }

    Ok(())
}

fn visit_expr(
    expr: &ExprAst,
    scope: &mut SymbolTable,
    mut module_builder: &mut ModuleBuilder,
) -> Result<(), GearsError> {
    match expr {
        ExprAst::Integer(e) => module_builder.load_int(*e),
        ExprAst::Bool(b) => module_builder.load_bool(b),
        ExprAst::If {
            cmp_expr,
            exprs,
            else_exprs,
        } => {
            visit_expr(cmp_expr, scope, &mut module_builder)?;
            let jump_index = module_builder.start_jump_if_false();
            let mut local_scope = (&scope).push();

            for stmt in exprs {
                visit_stmt(stmt.as_ref(), &mut local_scope, &mut module_builder)?;
            }

            match else_exprs {
                Some(exprs) => {
                    let jump_index = module_builder.start_else(jump_index);
                    let mut local_scope = (&scope).push();
                    for stmt in exprs {
                        visit_stmt(stmt.as_ref(), &mut local_scope, &mut module_builder)?;
                    }
                    module_builder.end_jump(jump_index);
                }
                None => {
                    module_builder.end_jump(jump_index);
                }
            }
        }
        ExprAst::Op(left, op, right) => {
            visit_expr(left, scope, &mut module_builder)?;
            visit_expr(right, scope, &mut module_builder)?;

            match op {
                BinOpAst::Add => module_builder.op_add(),
                BinOpAst::Sub => module_builder.op_sub(),
                BinOpAst::Mul => module_builder.op_mul(),
                BinOpAst::Div => module_builder.op_div(),
            }
        }
        ExprAst::FunctionCall { ref name, ref args } => {
            for arg in args {
                visit_expr(arg, scope, &mut module_builder)?;
            }

            let (maybe_symbol, is_global) = scope.resolve(name);

            match maybe_symbol {
                Some(symbol) => match symbol.get_type() {
                    &SymbolType::Function => {
                        if is_global {
                            module_builder.call_fn(*symbol.get_index(), args.len() as u8);
                        } else {
                            return Err(GearsError::InternalCompilerError(
                                "Closures are not supported yet".to_string(),
                            ));
                        }
                    }
                    &SymbolType::Variable => {
                        // TODO: return location
                        return Err(GearsError::ParseError {
                            location: lexer::Span::new(0,0),
                            message: format!("{} is not callable", name),
                        });
                    }
                },
                None => return Err(GearsError::SymbolNotFound(name.clone())),
            }
        }
        ExprAst::Variable(name) => {
            let (maybe_symbol, is_global) = scope.resolve(name);

            match maybe_symbol {
                Some(symbol) => match symbol.get_type() {
                    &SymbolType::Function => {
                        return Err(GearsError::InternalCompilerError(
                            "Functions are not first class yet".to_string(),
                        ))
                    }
                    &SymbolType::Variable => {
                        if !is_global {
                            module_builder.load_fast(*symbol.get_index());
                        }
                    }
                },
                None => return Err(GearsError::SymbolNotFound(name.clone())),
            }
        }
    }

    Ok(())
}
