use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

// Type Aliases
pub type Name = String;
pub type Args = Vec<ArgAst>;
pub type FnArgs = Vec<ExprAst>;
pub type ListArgs = Vec<Box<ExprAst>>;
pub type Stmts = (Vec<Box<StmtAst>>, Option<Box<StmtAst>>);

#[derive(Debug, Clone)]
pub enum ModStmtAst {
    FunctionDef {
        name: String,
        args: Args,
        exprs: Stmts,
        return_type: Vec<Name>,
    },
}

impl ModStmtAst {
    pub fn new_fn(
        name: String,
        args: Args,
        return_type: Option<Vec<Name>>,
        exprs: Stmts,
    ) -> ModStmtAst {
        let conv_return_type = match return_type {
            Some(e) => e,
            None => vec!["none".to_string()],
        };

        ModStmtAst::FunctionDef {
            name,
            args,
            exprs,
            return_type: conv_return_type,
        }
    }
}

#[derive(Debug, Clone)]
pub enum StmtAst {
    Expr(ExprAst),
    Assignment {
        new: bool,
        name: String,
        expr: ExprAst,
        types: Option<Vec<Name>>,
    },
}

impl StmtAst {
    pub fn new_assignment(name: String, types: Vec<Name>, expr: ExprAst) -> StmtAst {
        StmtAst::Assignment {
            new: true,
            name: name,
            expr: expr,
            types: Some(types),
        }
    }

    pub fn new_reassignment(name: String, expr: ExprAst) -> StmtAst {
        StmtAst::Assignment {
            new: false,
            name: name,
            expr: expr,
            types: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArgAst {
    name: Name,
    arg_types: Vec<Name>,
}

impl ArgAst {
    pub fn new(name: Name, arg_type: Vec<Name>) -> ArgAst {
        ArgAst {
            name: name,
            arg_types: arg_type,
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn arg_types(&self) -> &Vec<Name> {
        &self.arg_types
    }
}

#[derive(Copy, Clone)]
pub enum BinOpAst {
    Mul,
    Div,
    Add,
    Sub,
    EqEq,
    NotEq,
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
}

impl Debug for BinOpAst {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::BinOpAst::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            EqEq => write!(fmt, "=="),
            NotEq => write!(fmt, "!="),
            LessThan => write!(fmt, "<"),
            LessThanEq => write!(fmt, "<="),
            GreaterThan => write!(fmt, ">"),
            GreaterThanEq => write!(fmt, ">="),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExprAst {
    FunctionCall {
        name: Name,
        args: FnArgs,
    },
    Integer(i64),
    Op(Box<ExprAst>, BinOpAst, Box<ExprAst>),
    Variable(String),
    Bool(bool),
    Str(String),
    List(ListArgs),
    If {
        cmp_expr: Box<ExprAst>,
        exprs: Stmts,
        else_exprs: Option<Stmts>,
    },
    While {
        cmp_expr: Box<ExprAst>,
        exprs: Stmts,
    },
    For {
        name: String,
        range: RangeAst,
        exprs: Stmts,
    },
}

impl ExprAst {
    pub fn new_op(left: ExprAst, op: BinOpAst, right: ExprAst) -> ExprAst {
        ExprAst::Op(Box::new(left), op, Box::new(right))
    }

    pub fn new_if(cmp_expr: ExprAst, exprs: Stmts, else_exprs: Option<Stmts>) -> ExprAst {
        ExprAst::If {
            cmp_expr: Box::new(cmp_expr),
            exprs: exprs,
            else_exprs: else_exprs,
        }
    }

    pub fn new_while(cmp_expr: ExprAst, exprs: Stmts) -> ExprAst {
        ExprAst::While {
            cmp_expr: Box::new(cmp_expr),
            exprs: exprs,
        }
    }

    pub fn new_for(name: String, range: RangeAst, exprs: Stmts) -> ExprAst {
        ExprAst::For {
            name: name,
            range: range,
            exprs: exprs,
        }
    }

    pub fn new_list(exprs: FnArgs) -> ExprAst {
        ExprAst::List(
            exprs
                .into_iter()
                .map(|x| Box::new(x))
                .collect::<Vec<Box<ExprAst>>>(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct RangeAst {
    start: i64,
    end: i64,
}

impl RangeAst {
    pub fn new(start: i64, end: i64) -> RangeAst {
        RangeAst {
            start: start,
            end: end,
        }
    }

    pub fn start(&self) -> i64 {
        self.start
    }

    pub fn end(&self) -> i64 {
        self.end
    }
}
