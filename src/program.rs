use crate::environment::{value_string, Environment, Value};
use crate::parser;
use crate::types::Error;
use crate::types::Expr;
use crate::types::Expr::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Program {
    instr: Vec<Expr>,
}

impl Program {
    /// Constructs a new Program
    ///
    /// # Examples
    /// ```
    /// # use titleformat_rs::program::Program;
    /// let program = Program::new();
    /// ```
    pub fn new() -> Self {
        Program { instr: vec![] }
    }

    /// Parses a program string
    ///
    /// # Examples
    /// ```
    /// # use titleformat_rs::program::Program;
    /// let mut program = Program::new();
    /// assert_eq!(program.parse("[%artist%]").unwrap(), ());
    /// ```
    pub fn parse(&mut self, instr: &str) -> Result<(), Error> {
        self.instr = parser::parse(instr)?;
        Ok(())
    }

    /// Executes a program without any metadata
    ///
    /// # Examples
    /// ```
    /// # use titleformat_rs::program::Program;
    /// let mut program = Program::new();
    /// assert_eq!(program.parse("[%artist%]").unwrap(), ());
    /// assert_eq!(program.run().unwrap(), String::from(""));
    /// ```
    pub fn run(&mut self) -> Result<String, Error> {
        self.run_with_meta(HashMap::new())
    }

    /// Executes a program with associated metadata
    ///
    /// # Examples
    /// ```
    /// # use titleformat_rs::program::Program;
    /// # use std::collections::HashMap;
    /// let mut program = Program::new();
    /// assert_eq!(program.parse("[%artist%]").unwrap(), ());
    /// let mut metadata = HashMap::new();
    /// metadata.insert("artist".into(), vec!["Happy".into()]);
    /// assert_eq!(program.run_with_meta(metadata).unwrap(), String::from("Happy"));
    /// ```
    pub fn run_with_meta(&self, metadata: HashMap<String, Vec<String>>) -> Result<String, Error> {
        let mut env = Environment::new(metadata);
        let result = match self.resolve_arg_vec(&mut env, &self.instr)? {
            ExprValue(v) => v.val.clone(),
            _ => unreachable!(),
        };
        Ok(result)
    }

    /* resolves a set of expressions into a single resolved value
     * e.g. '%artist%literal' with artist=best would resolve to 'bestliteral' */
    fn resolve_arg_vec(&self, env: &mut Environment, args: &Vec<Expr>) -> Result<Expr, Error> {
        let mut new_arg = value_string("", false);

        for arg in args {
            let tmp = self.eval(env, arg)?;
            new_arg.val = new_arg.val + &tmp.val;
            /* picard does an or here */
            new_arg.cond = new_arg.cond || tmp.cond;
        }

        Ok(ExprValue(new_arg))
    }

    fn eval(&self, env: &mut Environment, expr: &Expr) -> Result<Value, Error> {
        match expr {
            ExprValue(v) => Ok(v.clone()),
            /* literals are always true for conditionals */
            Literal(v) => Ok(value_string(v, true)),
            Variable(var) => Ok(env.get_variable(var)),
            Conditional(args) => {
                let arg = self.resolve_arg_vec(env, args)?;
                match self.eval(env, &arg)? {
                    Value { val: v, cond: true } => Ok(value_string(&v, true)),
                    _ => Ok(value_string("", false)),
                }
            }
            FuncCall(name, args) => {
                let mut evaluated_args = Vec::new();
                for unresolved in args {
                    let resolved = self.resolve_arg_vec(env, unresolved)?;
                    let new_arg = self.eval(env, &resolved)?;
                    evaluated_args.push(new_arg);
                }
                Ok(env.call(name, evaluated_args)?)
            }
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
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![String::from("val")]);
        assert_eq!(prog.run_with_meta(m).unwrap(), String::from("val"));
    }

    #[test]
    fn test_run_unknown_variable() {
        let mut prog = Program::new();
        prog.parse("%unknown%").unwrap();
        assert_eq!(prog.run().unwrap(), String::from("?"));
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
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![String::from("2")]);
        assert_eq!(prog.run_with_meta(m).unwrap(), String::from("4"));
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
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![String::from("2")]);
        assert_eq!(prog.run_with_meta(m.clone()).unwrap(), String::from("4"));
        assert_eq!(prog.run_with_meta(m).unwrap(), String::from("4"));
    }

    #[test]
    fn test_run_conditional_variable_exists() {
        let mut prog = Program::new();
        prog.parse("[%a%]").unwrap();
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![String::from("val")]);
        assert_eq!(prog.run_with_meta(m).unwrap(), String::from("val"));
    }

    #[test]
    fn test_run_conditional_variable_nonexistent() {
        let mut prog = Program::new();
        prog.parse("[%a%]").unwrap();
        assert_eq!(prog.run().unwrap(), String::from(""));
    }

    #[test]
    fn test_run_conditional_function_variable_exists() {
        let mut prog = Program::new();
        prog.parse("[$add(%a%,2)]").unwrap();
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![String::from("2")]);
        assert_eq!(prog.run_with_meta(m).unwrap(), String::from("4"));
    }

    #[test]
    fn test_run_conditional_function_variable_nonexistent() {
        let mut prog = Program::new();
        prog.parse("[$add(%a%,2)]").unwrap();
        assert_eq!(prog.run().unwrap(), String::from(""));
    }

    #[test]
    fn test_conditional_variable_literal() {
        let mut prog = Program::new();
        prog.parse("[%a%b]").unwrap();
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![String::from("2")]);
        assert_eq!(prog.run_with_meta(m).unwrap(), String::from("2b"));
    }
}
