use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

// Type Aliases
type Name = String;
type Args = Vec<Arg>;
type Stmts = Vec<Box<Stmt>>;


#[derive(Debug)]
pub enum ModStmt {
    Actor {
        name: String,
        args: Args,
        exprs: Stmts,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Assignment { name: String, expr: Expr },
}

#[derive(Debug)]
pub struct Arg {
    name: Name,
}

impl Arg {
    pub fn new(name: Name) -> Arg{
        Arg {
            name: name,
        }
    }
}

#[derive(Copy, Clone)]
pub enum BinOp {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for BinOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::BinOp::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

pub enum Expr {
    Integer(i64),
    Op(Box<Expr>, BinOp, Box<Expr>),
    Variable(String),
}

impl Expr {
    pub fn new_op(left: Expr, op: BinOp, right: Expr) -> Expr {
        Expr::Op(Box::new(left), op, Box::new(right))
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Integer(n) => write!(fmt, "{:?}", n),
            Variable(ref s) => write!(fmt, "{}", s),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
        }
    }
}
