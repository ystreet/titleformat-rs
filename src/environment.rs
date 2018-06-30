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

#[derive(Clone)]
pub enum FuncValue {
  NativeFn(fn(Vec<String>) -> String),
  NativeFnError(fn(Vec<String>) -> Result<String, Error>),
  NativeCondFnError(fn(Vec<Value>) -> Result<Value, Error>),
  NativeEnvFnError(fn(&Environment, Vec<Value>) -> Result<Value, Error>),
}

/* everything is a string... */
/* like a register file in a cpu, but with strings! */
#[derive(Clone)]
pub struct Environment {
  vars     : HashMap<String, String>,
  metadata : HashMap<String, Vec<String>>,
  funcs    : HashMap<String, FuncValue>,
}

impl Environment {
  fn add_default_functions (&mut self) -> () {
    self.funcs.insert(String::from("add"), FuncValue::NativeFnError(functions::num::add::add));
    self.funcs.insert(String::from("sub"), FuncValue::NativeFnError(functions::num::sub::sub));
    self.funcs.insert(String::from("mul"), FuncValue::NativeFnError(functions::num::mul::mul));
    self.funcs.insert(String::from("div"), FuncValue::NativeFnError(functions::num::div::div));
    self.funcs.insert(String::from("min"), FuncValue::NativeFnError(functions::num::min::min));
    self.funcs.insert(String::from("max"), FuncValue::NativeFnError(functions::num::max::max));

    self.funcs.insert(String::from("if"), FuncValue::NativeCondFnError(functions::control::if_::if_));
    self.funcs.insert(String::from("if2"), FuncValue::NativeCondFnError(functions::control::if2::if2));
    self.funcs.insert(String::from("if3"), FuncValue::NativeCondFnError(functions::control::if3::if3));
    self.funcs.insert(String::from("ifequal"), FuncValue::NativeCondFnError(functions::control::ifequal::ifequal));
    self.funcs.insert(String::from("ifgreater"), FuncValue::NativeCondFnError(functions::control::ifgreater::ifgreater));
    self.funcs.insert(String::from("iflonger"), FuncValue::NativeCondFnError(functions::control::iflonger::iflonger));
    self.funcs.insert(String::from("select"), FuncValue::NativeCondFnError(functions::control::select::select));
    self.funcs.insert(String::from("and"), FuncValue::NativeCondFnError(functions::control::and::and));
    self.funcs.insert(String::from("or"), FuncValue::NativeCondFnError(functions::control::or::or));
    self.funcs.insert(String::from("xor"), FuncValue::NativeCondFnError(functions::control::xor::xor));
    self.funcs.insert(String::from("not"), FuncValue::NativeCondFnError(functions::control::not::not));

    self.funcs.insert(String::from("meta"), FuncValue::NativeEnvFnError(functions::env::meta::meta));
    self.funcs.insert(String::from("meta_sep"), FuncValue::NativeEnvFnError(functions::env::meta::meta_sep));
    self.funcs.insert(String::from("meta_num"), FuncValue::NativeEnvFnError(functions::env::meta::meta_num));
    self.funcs.insert(String::from("meta_test"), FuncValue::NativeEnvFnError(functions::env::meta::meta_test));
  }

  pub fn new(metadata : HashMap<String, Vec<String>>) -> Self {
    let mut env = Environment {
      vars     : HashMap::new(),
      metadata : metadata.clone(),
      funcs    : HashMap::new(),
    };
    Self::add_default_functions(&mut env);
    env
  }

  /* sets %key% to val and returns val */
  pub fn put(&mut self, key : &str, val : &str) -> String {
    let s = String::from(val);
    self.vars.insert(String::from(key), s.clone());
    s
  }

  /* sets %key% to val */
  pub fn puts(&mut self, key : &str, val : &str) {
    self.vars.insert(String::from(key), String::from(val));
  }

  /* get %key% */
  pub fn get(&self, key : &str) -> Value {
    match self.vars.get(key) {
      Some(v) => Value { val : v.clone(), cond : true },
      None => Value { val : String::from("?"), cond : false },
    }
  }

  pub fn meta_i(&self, key : &str, i : usize) -> Value {
    match self.metadata.get(key) {
      Some(v) => {
        if i >= v.len() {
          value_string("?", false)
        } else {
          value_string(&v[i], true)
        }
      },
      None => value_string("?", false),
    }
  }

  pub fn meta(&self, key : &str) -> Value {
    self.meta_sep(key, ", ")
  }

  pub fn meta_sep(&self, key : &str, sep : &str) -> Value {
    self.meta_sep_with_last(key, sep, sep)
  }

  pub fn meta_sep_with_last(&self, key : &str, sep : &str, last_sep : &str) -> Value {
    match self.metadata.get(key) {
      Some(v) => {
        let mut s = String::from("");
        for (i, val) in v.iter().enumerate() {
          if i > 0 && i+1 >= v.len() {
            s.push_str(last_sep);
          } else if i > 0 {
            s.push_str(sep);
          }
          s.push_str (&val);
        }
        value_string (&s, true)
      },
      None => value_string("?", false),
    }
  }

  pub fn meta_num(&self, key : &str) -> usize {
    match self.metadata.get(key) {
      Some(v) => v.len(),
      None => 0,
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
        FuncValue::NativeEnvFnError(func) => Ok(func(self, args)?),
      },
      None => Err(Error::UndefinedFunction(String::from(name))),
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_get() {
        let mut env = Environment::new(HashMap::new());
        assert_eq!(env.put("a", "val"), String::from("val"));
        assert_eq!(env.get("a"), value_string ("val", true));
    }

    #[test]
    fn test_puts_get() {
        let mut env = Environment::new(HashMap::new());
        env.puts("a", "val");
        assert_eq!(env.get("a"), value_string ("val", true));
    }

    #[test]
    fn test_get_unknown() {
        let env = Environment::new(HashMap::new());
        assert_eq!(env.get("invalid"), value_string("?", false));
    }

    #[test]
    fn test_call() {
        let env = Environment::new(HashMap::new());
        assert_eq!(env.call("add",
            vec![value_string("2", true), value_string("2", true)]).unwrap(),
            value_string("4", true));
    }

    #[test]
    fn test_call_unknown() {
        let env = Environment::new(HashMap::new());
        assert_eq!(env.call("unknown", vec![]).err().unwrap(),
            Error::UndefinedFunction(String::from("unknown")));
    }
}
