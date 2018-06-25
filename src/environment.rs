use std::collections::HashMap;

use types::Error;

use functions;

#[derive(Debug, Clone)]
pub enum FuncValue {
  NativeFn(fn(Vec<String>) -> String),
//  NativeFnError(fn(Vec<String>) -> Result<String, types::Error>),
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
    self.funcs.insert(String::from("add"), FuncValue::NativeFn(functions::add));
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
  pub fn get(&self, key : &str) -> &str {
    match self.vars.get(key) {
      Some(v) => v,
      None => "",
    }
  }

  pub fn call(&self, name : &str, args : Vec<String>) -> Result<String, Error> {
    match self.funcs.get(name) {
      Some(func_val) => match func_val {
        FuncValue::NativeFn(func) => Ok(func(args)),
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
        assert_eq!("val", env.get("a"));
    }

    #[test]
    fn test_get_unknown() {
        let env = Environment::new();
        assert_eq!("", env.get("invalid"));
    }

    #[test]
    fn test_call() {
        let env = Environment::new();
        assert_eq!(env.call("add", vec![String::from("2"), String::from("2")]).unwrap(), String::from("4"));
    }

    #[test]
    fn test_call_unknown() {
        let env = Environment::new();
        assert_eq!(env.call("unknown", vec![]).err().unwrap(), Error::UndefinedFunction(String::from("unknown")));
    }
}
