use ast::*;
use errors::GearsError;
use module::{Module, ModuleBuilder};
use parser;
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
    compile_ast(parser::ModuleParser::new().parse(string)?, name)
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
            ModStmtAst::FunctionDef { name, exprs, .. } => {
                module_builder.start_function(name.clone());
                let mut local_scope = (&symbol_table).push();

                for stmt in exprs {
                    visit_stmt(stmt.as_ref(), &mut local_scope, &mut module_builder)?;
                }

                module_builder.finish_function();
            }
        }
    }

    Ok(module_builder.build())
}

fn visit_stmt(stmt: &StmtAst, scope: &mut SymbolTable, mut module_builder: &mut ModuleBuilder) -> Result<(), GearsError> {
    match stmt {
        StmtAst::Expr(e) => visit_expr(e, scope, module_builder)?,
        StmtAst::Assignment{ name, expr } => { 
            visit_expr(expr, scope, &mut module_builder)?;
            let index = scope.def_variable(name.clone());
            module_builder.store_fast(index);
        }
    }

    Ok(())
} 

fn visit_expr(expr: &ExprAst, scope: &mut SymbolTable, mut module_builder: &mut ModuleBuilder) -> Result<(), GearsError>  {
    match expr {
        ExprAst::Integer(e) => module_builder.load_int(*e),
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
        ExprAst::Variable(name) => {
            let (maybe_symbol, is_global) = scope.resolve(name);
            
            match maybe_symbol {
                Some(symbol) => {
                    match symbol.get_type() {
                        &SymbolType::Function => return Err(GearsError::InternalCompilerError("Functions are callable yet".to_string())),
                        &SymbolType::Variable => {
                            if !is_global {
                                module_builder.load_fast(*symbol.get_index());
                            }
                        }
                    }
                },
                None => return Err(GearsError::SymbolNotFound(name.clone())),
            }
        } 
    }

    Ok(())
} 
