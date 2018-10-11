use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

// Type Aliases
type Name = String;
type Args = Vec<ArgAst>;
type FnArgs = Vec<ExprAst>;
type Stmts = Vec<Box<StmtAst>>;

#[derive(Debug, Clone)]
pub enum ModStmtAst {
    FunctionDef {
        name: String,
        args: Args,
        exprs: Stmts,
    },
}

#[derive(Debug, Clone)]
pub enum StmtAst {
    Expr(ExprAst),
    Assignment { name: String, expr: ExprAst },
}

#[derive(Debug, Clone)]
pub struct ArgAst {
    name: Name,
}

impl ArgAst {
    pub fn new(name: Name) -> ArgAst {
        ArgAst { name: name }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}

#[derive(Copy, Clone)]
pub enum BinOpAst {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for BinOpAst {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::BinOpAst::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExprAst {
    FunctionCall { name: Name, args: FnArgs },
    Integer(i64),
    Op(Box<ExprAst>, BinOpAst, Box<ExprAst>),
    Variable(String),
    Bool(bool),
    If{ cmp_expr: Box<ExprAst>, exprs: Stmts, else_exprs: Option<Stmts> },
}

impl ExprAst {
    pub fn new_op(left: ExprAst, op: BinOpAst, right: ExprAst) -> ExprAst {
        ExprAst::Op(Box::new(left), op, Box::new(right))
    }

    pub fn new_if(cmp_expr: ExprAst, exprs: Stmts, else_exprs: Option<Stmts>) -> ExprAst {
        ExprAst::If {
            cmp_expr: Box::new(cmp_expr),
            exprs: exprs,
            else_exprs: else_exprs
        }
    }
}
