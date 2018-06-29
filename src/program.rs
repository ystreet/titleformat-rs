use types::Error;
use types::Expr;
use types::Expr::*;
use parser;
use environment::Environment;

#[derive(Debug)]
pub struct Program {
  instr : Vec<Expr>,
  env : Environment,
}

impl Program {
  pub fn new() -> Self {
    let prog = Program {
      instr : vec![],
      env : Environment::new(),
    };
    prog
  }

  pub fn parse (&mut self, instr : &str) -> Result<(), Error> {
    self.instr = parser::parse(instr)?;
    self.env = Environment::new();
    Ok(())
  }

  pub fn run (&mut self) -> Result<String, Error> {
    let mut result = String::new();
    for e in &self.instr {
      result.push_str(&self.eval(&e)?);
    }
    Ok(result)
  }

  fn eval(&self, expr : &Expr) -> Result<String, Error> {
    match expr {
      Literal(v) => Ok(v.clone()),
      Variable(var) => Ok(self.env.get(&var).to_string()),
//      Conditional(c) => Ok("".to_string()), /* FIXME: */
      FuncCall(name, args) => {
        let mut evaluated_args = Vec::new();
        for arg in args {
          let mut new_arg = self.eval(arg)?;
          evaluated_args.push(String::from(new_arg));
        };
        Ok(self.env.call(&name, evaluated_args)?.to_string())
      },
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut prog = Program::new();
        prog.parse("%a%").unwrap();
    }

    #[test]
    fn test_multi_parse() {
        let mut prog = Program::new();
        prog.parse("%a%").unwrap();
        prog.parse("%b%").unwrap();
    }

    #[test]
    fn test_run_empty() {
        let mut prog = Program::new();
        prog.parse("").unwrap();
        assert_eq!(prog.run().unwrap(), String::from(""));
    }

    #[test]
    fn test_run() {
        let mut prog = Program::new();
        prog.parse("%a%").unwrap();
        prog.env.set("a", "val");
        assert_eq!(prog.run().unwrap(), String::from("val"));
    }

    #[test]
    fn test_run_unknown_variable() {
        let mut prog = Program::new();
        prog.parse("%unknown%").unwrap();
        assert_eq!(prog.run().unwrap(), String::from(""));
    }

    #[test]
    fn test_run_func() {
        let mut prog = Program::new();
        prog.parse("$add(2,2)").unwrap();
        assert_eq!(prog.run().unwrap(), String::from("4"));
    }

    #[test]
    fn test_run_func_variable() {
        let mut prog = Program::new();
        prog.parse("$add(%a%,2)").unwrap();
        prog.env.set("a", "2");
        assert_eq!(prog.run().unwrap(), String::from("4"));
    }

    #[test]
    fn test_run_func_func() {
        let mut prog = Program::new();
        prog.parse("$add($add(1,1),2)").unwrap();
        assert_eq!(prog.run().unwrap(), String::from("4"));
    }

    #[test]
    fn test_multi_run() {
        let mut prog = Program::new();
        prog.parse("$add(%a%,2)").unwrap();
        prog.env.set("a", "2");
        assert_eq!(prog.run().unwrap(), String::from("4"));
        assert_eq!(prog.run().unwrap(), String::from("4"));
    }
}
