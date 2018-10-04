use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

// Type Aliases
type Name = String;
type Args = Vec<ArgAst>;
type Stmts = Vec<Box<StmtAst>>;

#[derive(Debug)]
pub enum ModStmtAst {
    FunctionDef {
        name: String,
        args: Args,
        exprs: Stmts,
    },
}

#[derive(Debug)]
pub enum StmtAst {
    Expr(ExprAst),
    Assignment { name: String, expr: ExprAst },
}

#[derive(Debug)]
pub struct ArgAst {
    name: Name,
}

impl ArgAst {
    pub fn new(name: Name) -> ArgAst {
        ArgAst { name: name }
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

pub enum ExprAst {
    Integer(i64),
    Op(Box<ExprAst>, BinOpAst, Box<ExprAst>),
    Variable(String),
}

impl ExprAst {
    pub fn new_op(left: ExprAst, op: BinOpAst, right: ExprAst) -> ExprAst {
        ExprAst::Op(Box::new(left), op, Box::new(right))
    }
}

impl Debug for ExprAst {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::ExprAst::*;
        match *self {
            Integer(n) => write!(fmt, "{:?}", n),
            Variable(ref s) => write!(fmt, "{}", s),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
        }
    }
}
