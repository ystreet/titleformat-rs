use std::collections::HashMap;

use types::Error;

use functions;

#[derive(Debug, PartialEq, Clone)]
pub struct Value {
  pub val : String,
  pub cond : bool,
}

pub fn value_string(s : &str, c : bool) -> Value {
    Value { val : String::from(s), cond : c }
}

#[derive(Debug, Clone)]
pub enum FuncValue {
  NativeFn(fn(Vec<String>) -> String),
  NativeFnError(fn(Vec<String>) -> Result<String, Error>),
  NativeCondFnError(fn(Vec<Value>) -> Result<Value, Error>),
}

/* everything is a string... */
/* like a register file in a cpu, but with strings! */
#[derive(Debug, Clone)]
pub struct Environment {
  vars : HashMap<String, String>,
  funcs : HashMap<String, FuncValue>,
}

impl Environment {
  fn add_default_functions (&mut self) -> () {
    self.funcs.insert(String::from("add"), FuncValue::NativeFnError(functions::add));
    self.funcs.insert(String::from("sub"), FuncValue::NativeFnError(functions::sub));
    self.funcs.insert(String::from("div"), FuncValue::NativeFnError(functions::div));
    self.funcs.insert(String::from("mul"), FuncValue::NativeFnError(functions::mul));
    self.funcs.insert(String::from("max"), FuncValue::NativeFnError(functions::max));
    self.funcs.insert(String::from("min"), FuncValue::NativeFnError(functions::min));
  }

  pub fn new() -> Self {
    let mut env = Environment {
      vars : HashMap::new(),
      funcs: HashMap::new(),
    };
    Self::add_default_functions(&mut env);
    env
  }

  /* sets %key% to val */
  pub fn set(&mut self, key : &str, val : &str) {
    self.vars.insert(String::from(key), String::from(val));
  }

  /* sets %key% to val */
  pub fn get(&self, key : &str) -> Value {
    match self.vars.get(key) {
      Some(v) => Value { val : v.clone(), cond : true },
      None => Value { val : String::from("?"), cond : false },
    }
  }

  pub fn call(&self, name : &str, args : Vec<Value>) -> Result<Value, Error> {
    match self.funcs.get(name) {
      Some(func_val) => match func_val {
        FuncValue::NativeFn(func) => {
            /* return true only if all inputs are true */
            let c = args.iter().fold(true, |c, val| { c && val.cond });
            Ok(Value {
                val : func(args.iter().map(|a| { a.val.clone() }).collect()),
                cond : c
            })
        },
        FuncValue::NativeFnError(func) => {
            /* return true only if all inputs are true */
            let c = args.iter().fold(true, |c, val| { c && val.cond });
            Ok(Value {
                val : func(args.iter().map(|a| { a.val.clone() }).collect())?,
                cond : c
            })
        },
        FuncValue::NativeCondFnError(func) => Ok(func(args)?),
      },
      None => Err(Error::UndefinedFunction(String::from(name))),
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get() {
        let mut env = Environment::new();
        env.set("a", "val");
        assert_eq!(env.get("a"), value_string ("val", true));
    }

    #[test]
    fn test_get_unknown() {
        let env = Environment::new();
        assert_eq!(env.get("invalid"), value_string("?", false));
    }

    #[test]
    fn test_call() {
        let env = Environment::new();
        assert_eq!(env.call("add",
            vec![value_string("2", true), value_string("2", true)]).unwrap(),
            value_string("4", true));
    }

    #[test]
    fn test_call_unknown() {
        let env = Environment::new();
        assert_eq!(env.call("unknown", vec![]).err().unwrap(),
            Error::UndefinedFunction(String::from("unknown")));
    }
}
