use ast::*;
use errors::GearsError;
use lexer;
use module::{Module, ModuleBuilder};
use parser;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use symbol::{SymbolTable, SymbolType, Type, Types};

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
            ModStmtAst::FunctionDef {
                name,
                args,
                return_type,
                ..
            } => {
                let mut arg_types = HashMap::new();

                for arg in args {
                    arg_types.insert(arg.name().clone(), compile_types(arg.arg_types()));
                }

                symbol_table.def_fn(name.clone(), arg_types, compile_types(return_type));
            }
        }
    }

    for ref mod_stmt in &ast {
        match mod_stmt.as_ref() {
            ModStmtAst::FunctionDef {
                name, exprs, args, ..
            } => {
                module_builder.start_function(name.clone(), args.len());
                let mut local_scope = (&symbol_table).push();

                for arg in args {
                    local_scope.def_variable(arg.name().clone(), compile_types(arg.arg_types()));
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

fn compile_types(types: &Vec<String>) -> Vec<Type> {
    let mut types_vec = Vec::new();

    for t in types {
        types_vec.push(Type::from(t.clone()))
    }
    types_vec
}

fn visit_block(
    exprs: &Vec<Box<StmtAst>>,
    scope: &mut SymbolTable,
    mut module_builder: &mut ModuleBuilder,
) -> Result<Types, GearsError> {
    let mut local_scope = (&scope).push();
    let mut last_type = vec![Type::new_none()];

    for stmt in exprs {
        last_type = visit_stmt(stmt, &mut local_scope, &mut module_builder)?;
    }
    Ok(last_type)
}

fn visit_stmt(
    stmt: &StmtAst,
    scope: &mut SymbolTable,
    mut module_builder: &mut ModuleBuilder,
) -> Result<Types, GearsError> {
    match stmt {
        StmtAst::Expr(e) => visit_expr(e, scope, module_builder),
        StmtAst::Assignment {
            name,
            expr,
            new,
            types,
        } => {
            let expr_types = visit_expr(expr, scope, &mut module_builder)?;

            let var_types: Types = match types {
                Some(given_types) => {
                    let given_types = compile_types(&given_types);
                    for expr_type in &expr_types {
                        if !given_types.contains(&expr_type) {
                            return Err(GearsError::TypeError(format!(
                                "{:?} is not compatible with {:?}",
                                given_types, expr_types
                            )));
                        }
                    }
                    given_types
                }
                None => expr_types,
            };

            let index = if *new {
                scope.def_variable(name.clone(), compile_types(&types.clone().unwrap()))
            } else {
                let (symbol, _is_global) = scope.resolve(name);

                match symbol {
                    Some(e) => *e.get_index(),
                    None => return Err(GearsError::SymbolNotFound(name.clone())),
                }
            };

            module_builder.store_fast(index);
            Ok(var_types)
        }
    }
}

fn visit_expr(
    expr: &ExprAst,
    scope: &mut SymbolTable,
    mut module_builder: &mut ModuleBuilder,
) -> Result<Types, GearsError> {
    let res: Types = match expr {
        ExprAst::Integer(e) => {
            module_builder.load_int(*e);
            vec![Type::new_int()]
        }
        ExprAst::Bool(b) => {
            module_builder.load_bool(b);
            vec![Type::new_bool()]
        }
        ExprAst::List(_) => panic!("List is not added yet"),
        ExprAst::If {
            cmp_expr,
            exprs,
            else_exprs,
        } => {
            visit_expr(cmp_expr, scope, &mut module_builder)?;
            let jump_index = module_builder.start_jump_if_false();
            let if_block_types =
                HashSet::from_iter(visit_block(exprs, scope, &mut module_builder)?);

            let else_block_types: HashSet<Type> = match else_exprs {
                Some(exprs) => {
                    let jump_index = module_builder.start_else(jump_index);
                    let mut local_scope = (&scope).push();
                    let mut last_type = vec![Type::new_none()];
                    for stmt in exprs {
                        last_type =
                            visit_stmt(stmt.as_ref(), &mut local_scope, &mut module_builder)?;
                    }
                    module_builder.end_jump(jump_index);
                    HashSet::from_iter(last_type)
                }
                None => {
                    let jump_index = module_builder.start_else(jump_index);
                    module_builder.load_none();
                    module_builder.end_jump(jump_index);
                    let mut res = HashSet::new();
                    res.insert(Type::new_none());
                    res
                }
            };

            let mut res = Vec::new();
            for res_type in if_block_types.union(&else_block_types) {
                res.push(res_type.clone());
            }

            res
        }
        ExprAst::While { cmp_expr, exprs } => {
            let loop_index = module_builder.start_loop_check();
            visit_expr(cmp_expr, scope, &mut module_builder)?;
            let jump_index = module_builder.start_jump_if_false();
            let result = visit_block(exprs, scope, &mut module_builder)?;
            module_builder.end_loop(loop_index, jump_index);
            result
        }
        ExprAst::For { name, range, exprs } => {
            // Push a loop level scope and define the name into that scope
            let mut local_scope = (&scope).push();
            let name_index = local_scope.def_variable(name.clone(), vec![Type::from("int")]);
            module_builder.load_int(range.start());
            module_builder.store_fast(name_index);
            let loop_index = module_builder.start_loop_check();

            let cmp_expr = &ExprAst::new_op(
                ExprAst::Variable(name.clone()),
                BinOpAst::LessThan,
                ExprAst::Integer(range.end()),
            );

            visit_expr(cmp_expr, &mut local_scope, &mut module_builder)?;

            let jump_index = module_builder.start_jump_if_false();
            let result = visit_block(exprs, &mut local_scope, &mut module_builder)?;
            module_builder.load_fast(name_index);
            module_builder.inc_one();
            module_builder.store_fast(name_index);
            module_builder.end_loop(loop_index, jump_index);
            result
        }
        ExprAst::Op(left, op, right) => {
            use self::BinOpAst::*;

            let left_types = visit_expr(left, scope, &mut module_builder)?;
            let right_types = visit_expr(right, scope, &mut module_builder)?;

            match op {
                BinOpAst::Add => module_builder.op_add(),
                BinOpAst::Sub => module_builder.op_sub(),
                BinOpAst::Mul => module_builder.op_mul(),
                BinOpAst::Div => module_builder.op_div(),
                BinOpAst::EqEq => module_builder.op_eqeq(),
                BinOpAst::NotEq => module_builder.op_not_eq(),
                BinOpAst::LessThan => module_builder.op_less(),
                BinOpAst::LessThanEq => module_builder.op_less_eq(),
                BinOpAst::GreaterThan => module_builder.op_greater(),
                BinOpAst::GreaterThanEq => module_builder.op_greater_eq(),
            }

            match op {
                Add | Sub | Mul | Div => {
                    if left_types.len() != 1 || left_types[0] != Type::new_int() {
                        return Err(GearsError::TypeError(format!(
                            "Only ints are supported for math at this time. Left hand is: {:?}",
                            left_types
                        )));
                    }
                    if right_types.len() != 1 || right_types[0] != Type::new_int() {
                        return Err(GearsError::TypeError(format!(
                            "Only ints are supported for math at this time. Right hand is: {:?}",
                            right_types
                        )));
                    }

                    vec![Type::new_int()]
                }
                _ => vec![Type::new_bool()],
            }
        }
        ExprAst::FunctionCall { ref name, ref args } => {
            for arg in args {
                visit_expr(arg, scope, &mut module_builder)?;
            }

            let (maybe_symbol, is_global) = scope.resolve(name);

            match maybe_symbol {
                Some(symbol) => match symbol.get_type() {
                    &SymbolType::Function {
                        ref return_types, ..
                    } => {
                        if is_global {
                            module_builder.call_fn(*symbol.get_index(), args.len() as u8);
                            return_types.clone()
                        } else {
                            return Err(GearsError::InternalCompilerError(
                                "Closures are not supported yet".to_string(),
                            ));
                        }
                    }
                    &SymbolType::Variable { .. } => {
                        // TODO: return location
                        return Err(GearsError::ParseError {
                            location: lexer::Span::new(0, 0),
                            message: format!("{} is not callable", name),
                        });
                    }
                },
                None => return Err(GearsError::SymbolNotFound(name.clone())),
            }
        }
        ExprAst::Variable(name) => {
            let (maybe_symbol, _) = scope.resolve(name);

            match maybe_symbol {
                Some(symbol) => match symbol.get_type() {
                    &SymbolType::Function { .. } => {
                        return Err(GearsError::InternalCompilerError(
                            "Functions are not first class yet".to_string(),
                        ))
                    }
                    &SymbolType::Variable { ref types } => {
                        module_builder.load_fast(*symbol.get_index());
                        types.clone()
                    }
                },
                None => return Err(GearsError::SymbolNotFound(name.clone())),
            }
        }
    };

    Ok(res)
}
