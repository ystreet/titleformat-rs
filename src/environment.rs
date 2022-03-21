use std::collections::HashMap;

use types::Error;
use types::Error::*;
use std::cell::RefCell;

use functions;
use functions::num::to_int;

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
  vars     : RefCell<HashMap<String, String>>,
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

    self.funcs.insert(String::from("eq"), FuncValue::NativeCondFnError(functions::num::control::eq));
    self.funcs.insert(String::from("ne"), FuncValue::NativeCondFnError(functions::num::control::ne));
    self.funcs.insert(String::from("gt"), FuncValue::NativeCondFnError(functions::num::control::gt));
    self.funcs.insert(String::from("gte"), FuncValue::NativeCondFnError(functions::num::control::gte));
    self.funcs.insert(String::from("lt"), FuncValue::NativeCondFnError(functions::num::control::lt));
    self.funcs.insert(String::from("lte"), FuncValue::NativeCondFnError(functions::num::control::lte));

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

    self.funcs.insert(String::from("crlf"), FuncValue::NativeCondFnError(functions::str::constants::crlf));
    self.funcs.insert(String::from("tab"), FuncValue::NativeCondFnError(functions::str::constants::tab));
    self.funcs.insert(String::from("noop"), FuncValue::NativeCondFnError(functions::str::constants::noop));

    self.funcs.insert(String::from("upper"), FuncValue::NativeCondFnError(functions::str::case::upper));
    self.funcs.insert(String::from("lower"), FuncValue::NativeCondFnError(functions::str::case::lower));
    self.funcs.insert(String::from("firstalphachar"), FuncValue::NativeCondFnError(functions::str::case::firstalphachar));

    self.funcs.insert(String::from("len"), FuncValue::NativeCondFnError(functions::str::size::len));
    self.funcs.insert(String::from("longer"), FuncValue::NativeCondFnError(functions::str::size::longer));

    self.funcs.insert(String::from("stripprefix"), FuncValue::NativeCondFnError(functions::str::modify::stripprefix));
    self.funcs.insert(String::from("swapprefix"), FuncValue::NativeCondFnError(functions::str::modify::swapprefix));
    self.funcs.insert(String::from("cut"), FuncValue::NativeCondFnError(functions::str::modify::cut));
    self.funcs.insert(String::from("left"), FuncValue::NativeCondFnError(functions::str::modify::left));
    self.funcs.insert(String::from("num"), FuncValue::NativeCondFnError(functions::str::format::num));

    self.funcs.insert(String::from("meta"), FuncValue::NativeEnvFnError(Environment::meta_value));
    self.funcs.insert(String::from("meta_sep"), FuncValue::NativeEnvFnError(Environment::meta_sep_value));
    self.funcs.insert(String::from("meta_num"), FuncValue::NativeEnvFnError(Environment::meta_num_value));
    self.funcs.insert(String::from("meta_test"), FuncValue::NativeEnvFnError(Environment::meta_test_value));
    self.funcs.insert(String::from("get"), FuncValue::NativeEnvFnError(Environment::get_value));
    self.funcs.insert(String::from("put"), FuncValue::NativeEnvFnError(Environment::put_value));
    self.funcs.insert(String::from("puts"), FuncValue::NativeEnvFnError(Environment::puts_value));

    self.funcs.insert(String::from("year"), FuncValue::NativeCondFnError(functions::str::datetime::year));
  }

  /// Constructs a new `Environment`
  ///
  /// # Examples
  ///
  /// Constructing an `Environment` without any metadata
  ///
  /// ```
  /// # use titleformat_rs::environment::Environment;
  /// # use std::collections::HashMap;
  /// let env = Environment::new(HashMap::new());
  /// ```
  ///
  /// Constructing an `Environment` with various metadata
  ///
  /// ```
  /// # use titleformat_rs::environment::Environment;
  /// # use std::collections::HashMap;
  /// let mut metadata = HashMap::new();
  /// metadata.insert(String::from("key"), vec![String::from("value1"), String::from("value2")]);
  /// let env = Environment::new(metadata);
  /// ```
  pub fn new(metadata : HashMap<String, Vec<String>>) -> Self {
    let mut env = Environment {
      vars     : RefCell::new(HashMap::new()),
      metadata : metadata.clone(),
      funcs    : HashMap::new(),
    };
    Self::add_default_functions(&mut env);
    env
  }

  fn put_value(&self, args : Vec<Value>) -> Result<Value, Error> {
    match args.len() {
      2 => Ok(value_string(&self.put(&args[0].val, &args[1].val), true)),
      _ => Err(InvalidNativeFunctionArgs(String::from("put"), args.len())),
    }
  }

  /* sets %key% to val and returns val */
  fn put(&self, key : &str, val : &str) -> String {
    let s = String::from(val);
    self.vars.borrow_mut().insert(String::from(key), s.clone());
    s
  }

  fn puts_value(&self, args : Vec<Value>) -> Result<Value, Error> {
    match args.len() {
      2 => Ok(value_string({ self.puts(&args[0].val, &args[1].val); "" }, true)),
      _ => Err(InvalidNativeFunctionArgs(String::from("puts"), args.len())),
    }
  }

  /* sets key to val inside vars */
  fn puts(&self, key : &str, val : &str) {
    self.vars.borrow_mut().insert(String::from(key), String::from(val));
  }

  fn get_value(&self, args : Vec<Value>) -> Result<Value, Error> {
    match args.len() {
      1 => Ok(self.get(&args[0].val)),
      _ => Err(InvalidNativeFunctionArgs(String::from("get"), args.len())),
    }
  }

  /* sets key from vars */
  fn get(&self, key : &str) -> Value {
    match self.vars.borrow().get(key) {
      Some(v) => Value { val : v.clone(), cond : true },
      None => Value { val : String::from("?"), cond : false },
    }
  }

  pub fn get_variable(&self, key : &str) -> Value {
    self.meta_i(key, 0)
  }

  /* gets the ith key from the metadata */
  fn meta_i(&self, key : &str, i : usize) -> Value {
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

  /* gets the key from the metadata separated by ", " */
  fn meta(&self, key : &str) -> Value {
    self.meta_sep(key, ", ")
  }

  /* gets the key from the metadata separated by sep */
  fn meta_sep(&self, key : &str, sep : &str) -> Value {
    self.meta_sep_with_last(key, sep, sep)
  }

  /* gets the key from the metadata separated by ", " and last separator with last_sep */
  fn meta_sep_with_last(&self, key : &str, sep : &str, last_sep : &str) -> Value {
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

  fn meta_num(&self, key : &str) -> usize {
    match self.metadata.get(key) {
      Some(v) => v.len(),
      None => 0,
    }
  }

/*
 * $meta(name)
 * Returns value of tag called name. If multiple values of that tag exist,
 * they are concatenated with ", " as separator.
 * Example: $meta(artist) → "He, She, It"
 *
 * $meta(name,n)
 * Returns value of n-th (0,1,2 and so on) tag called name.
 * Example: $meta(artist,1) → "She" 
 */
  fn meta_value(&self, args : Vec<Value>) -> Result<Value, Error> {
    match args.len() {
      1 => Ok(self.meta(&args[0].val)),
      2 => Ok(self.meta_i(&args[0].val, to_int(&args[1].val) as usize)),
      _ => Err(InvalidNativeFunctionArgs(String::from("meta"), args.len())),
    }
  }

/*
 * $meta_sep(name,sep)
 * Returns value of tag called name. If multiple values of that tag exist, they are concatenated with sep as separator.
 * Example: $meta_sep(artist,' + ') → "He + She + It"
 *
 * $meta_sep(name,sep,lastsep)
 * Returns value of tag called name. If multiple values of that tag exist,
 * they are concatenated with sep as separator between all but the last two
 * values which are concatenated with lastsep.
 * Example: $meta_sep(artist,', ',', and ') → "He, She, and It"
 */
  fn meta_sep_value (&self, args : Vec<Value>) -> Result<Value, Error> {
    match args.len() {
      2 => Ok(self.meta_sep(&args[0].val, &args[1].val)),
      3 => Ok(self.meta_sep_with_last(&args[0].val, &args[1].val, &args[2].val)),
      _ => Err(InvalidNativeFunctionArgs(String::from("meta_sep"), args.len())),
    }
  }

/*
 * $meta_test(...)
 * Returns 1, if all given tags exist, undefined otherwise.
 * Example: $meta_test(artist,title) → true
 */
  fn meta_test_value (&self, args : Vec<Value>) -> Result<Value, Error> {
    match args.len() {
      0 => Err(InvalidNativeFunctionArgs(String::from("meta_num"), args.len())),
      _ => Ok(value_string("", args.iter().fold(true, |c, i| {
        c && self.meta_num(&i.val) > 0
      }))),
    }
  }

/*
 * $meta_num(name)
 * Returns the number of values for the tag called name.
 * Example: $meta_num(artist) → 3
 */
  fn meta_num_value (&self, args : Vec<Value>) -> Result<Value, Error> {
    match args.len() {
      1 => Ok(value_string(&self.meta_num(&args[0].val).to_string(), true)),
      _ => Err(InvalidNativeFunctionArgs(String::from("meta_num"), args.len())),
    }
  }

  pub fn call(&self, name : &str, args : Vec<Value>) -> Result<Value, Error> {
    let f = {
        self.funcs.get(name)
    };
    match f {
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
        let env = Environment::new(HashMap::new());
        assert_eq!(env.put("a", "val"), String::from("val"));
        assert_eq!(env.get("a"), value_string ("val", true));
    }

    #[test]
    fn test_put_get_value() {
        let env = Environment::new(HashMap::new());

        assert_eq!(env.put_value(vec![
                value_string("a", true),
                value_string("val", true)
            ]).unwrap(),
            value_string("val", true));
        assert_eq!(
            env.get_value(vec![value_string("a", true)]).unwrap(),
            value_string ("val", true)
        );

        assert_eq!(env.puts_value(vec![
                value_string("b", true),
                value_string("bar", true)
            ]).unwrap(),
            value_string("", true)
        );
        assert_eq!(
            env.get_value(vec![value_string("b", true)]).unwrap(),
            value_string ("bar", true)
        );
    }

    #[test]
    fn test_puts_get() {
        let env = Environment::new(HashMap::new());
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

    #[test]
    fn wrong_n_arguments() {
        let env = Environment::new(HashMap::new());
        assert_eq!(env.meta_value(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("meta"), 0));
        assert_eq!(env.meta_sep_value(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("meta_sep"), 0));
        assert_eq!(env.meta_num_value(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("meta_num"), 0));
        assert_eq!(env.put_value(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("put"), 0));
        assert_eq!(env.puts_value(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("puts"), 0));
        assert_eq!(env.get_value(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("get"), 0));
    }

    #[test]
    fn test_meta() {
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![
            String::from("0"),
            String::from("1"),
            String::from("2"),
            String::from("3")
        ]);
        let env = Environment::new(m);
        assert_eq!(env.meta_value(
                vec![
                    value_string("a", true)
                ]
            ).unwrap(),
            value_string("0, 1, 2, 3", true)
        );
        assert_eq!(env.meta_value(
                vec![
                    value_string("a", true),
                    value_string("1", true)
                ]
            ).unwrap(),
            value_string("1", true)
        );
        assert_eq!(env.meta_value(
                vec![
                    value_string("a", true),
                    value_string("1000", true)
                ]
            ).unwrap(),
            value_string("?", false)
        );
    }

    #[test]
    fn test_meta_sep() {
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![
            String::from("0"),
            String::from("1"),
            String::from("2"),
            String::from("3")
        ]);
        let env = Environment::new(m);
        assert_eq!(env.meta_sep_value(
                vec![
                    value_string("a", true),
                    value_string("|", true)
                ]
            ).unwrap(),
            value_string("0|1|2|3", true)
        );
        assert_eq!(env.meta_sep_value(
                vec![
                    value_string("a", true),
                    value_string("|", true),
                    value_string("^", true)
                ]
            ).unwrap(),
            value_string("0|1|2^3", true)
        );
    }

    #[test]
    fn test_meta_num() {
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![
            String::from("0"),
            String::from("1"),
            String::from("2"),
            String::from("3")
        ]);
        let env = Environment::new(m);
        assert_eq!(env.meta_num_value(
                vec![
                    value_string("a", true)
                ]
            ).unwrap(),
            value_string("4", true)
        );
        assert_eq!(env.meta_num_value(
                vec![
                    value_string("unknown", true)
                ]
            ).unwrap(),
            value_string("0", true)
        );
    }

    #[test]
    fn test_meta_test() {
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![
            String::from("0"),
            String::from("1"),
            String::from("2"),
            String::from("3")
        ]);
        m.insert(String::from("b"), vec![
            String::from("4"),
        ]);
        let env = Environment::new(m);
        assert_eq!(env.meta_test_value(
                vec![
                    value_string("a", true),
                    value_string("b", true)
                ]
            ).unwrap(),
            value_string("", true)
        );
        assert_eq!(env.meta_test_value(
                vec![
                    value_string("unknown", true),
                ]
            ).unwrap(),
            value_string("", false)
        );
    }
}
