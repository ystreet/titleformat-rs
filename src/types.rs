use std::collections::HashMap;
use std::fmt;
use std::result;

pub type Result = result::Result<Expr, Error>;

#[derive(Debug)]
pub enum Error {
    UndefinedVariable(String),
    InvalidNativeFunctionArgs(String, usize),
    UndefinedFunction(String),
    ParseError,
}

use types::Error::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UndefinedVariable(ref varname) => write!(f, "Undefined Variable: {}", varname),
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
  Nil,
  Literal(String),
  Variable(String),
  Conditional(Box<Expr>), /* [expression] */
  FuncCall(String, Vec<Expr>),
  /* XXX: comments */
}

pub enum FuncValue {
  NativeFn(fn(Vec<String>) -> String),
}

/* everything is a string... */
/* like a register file in a cpu, but with strings! */
#[derive(Clone)]
pub struct Environment {
  vars : HashMap<String, String>,
  funcs : HashMap<String, Vec<Expr>>,
}

impl Environment {
  pub fn new() -> Self {
    let mut env = Environment {
      vars : HashMap::new(),
      funcs: HashMap::new(),
    };
//    add_default_functions(env);
    env
  }

  fn add_default_functions (&mut self) -> () {
/* FIXME:
    self.funcs.insert(String::from("add"),
      NativeFnVariable(|x| -> String {
        
      }
    );
  */
  }
}
