use std::fmt;
use std::result;

pub type Result = result::Result<Expr, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidNativeFunctionArgs(String, usize),
    UndefinedFunction(String),
    ParseError,
}

use types::Error::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidNativeFunctionArgs(ref native_fn_name, ref actual) => {
                write!(
                    f,
                    "Syntax Error: Native function '{}' called with the wrong number of arguments {}",
                    native_fn_name,
                    actual
                )
            }
            UndefinedFunction(ref varname) => write!(f, "Undefined Function: {}", varname),
            ParseError => write!(f, "Unable the parse the input. Please recheck."),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
  Literal(String),
  Variable(String),             /* %variable% */
//  Conditional(Box<Expr>),       /* [expression] */
  FuncCall(String, Vec<Expr>),  /* $func(args) */
  /* XXX: comments */
}

