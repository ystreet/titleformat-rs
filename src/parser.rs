use nom::{digit, alphanumeric};
use nom::types::CompleteStr;

use types::Expr;
use types::Expr::*;
use types::Error;
use types::Error::*;

named!(varname<CompleteStr, CompleteStr>, ws!(alphanumeric));

/* %varname% */
named!(variable<CompleteStr, Expr>,
    do_parse!(
        char!('%') >>
        var_name : varname >>
        char!('%') >>
        (parse_varname(&var_name))
));

/* $funcname(arg1, arg2) */
named!(func<CompleteStr, Expr>,
    do_parse!(
        char!('$') >>
        func_name: varname >>
        args : ws!(delimited!(char!('('), separated_list!(char!(','), nested_expr), char!(')'))) >>
        (parse_funccall(&func_name, args))
));
named!(literal<CompleteStr, Expr>,
    do_parse!(
        lit : alphanumeric >>
        (parse_literal (&lit))
));

named!(nested_expr<CompleteStr, Expr>, alt!(func | variable | literal));
named!(expr<CompleteStr, Vec<Expr>>, many0!(nested_expr));

pub fn parse(input: &str) -> Result<Vec<Expr>, Error> {
    match expr(CompleteStr(input)) {
        Ok((_, expr)) => Ok(expr),
        e => { println!("{:?}", e); Err(ParseError)},
    }
}

fn parse_literal(literal: &str) -> Expr {
    Literal(String::from(literal))
}

fn parse_varname(name: &str) -> Expr {
    Variable(String::from(name))
}

fn parse_funccall(name: &str, args : Vec<Expr>) -> Expr {
    FuncCall(String::from(name), args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable() {
        let parsed = parse("%ab%").unwrap();
        assert_eq!(parsed, vec![
            Variable(String::from("ab"))
        ]);
    }

    #[test]
    fn test_empty_funccall() {
        let parsed = parse("$ab()").unwrap();
        assert_eq!(parsed, vec![
            FuncCall(
                String::from("ab"),
                vec![]
            )
        ]);
    }

    #[test]
    fn test_funccall_variable() {
        let parsed = parse("$ab(%ba%)").unwrap();
        assert_eq!(parsed, vec![
            FuncCall(
                String::from("ab"),
                vec![
                    Variable(String::from("ba"))
                ]
            )
        ]);
    }

    #[test]
    fn test_funccall_funccall() {
        let parsed = parse("$ab($ba())").unwrap();
        assert_eq!(parsed, vec![
            FuncCall(
                String::from("ab"),
                vec![
                    FuncCall(String::from("ba"), vec![])
                ]
            )
        ]);
    }

    #[test]
    fn test_funccall_complex() {
        let parsed = parse("$ab($cd(%e%,fg),hi)").unwrap();
        assert_eq!(parsed, vec![
            FuncCall(
                String::from("ab"),
                vec![
                    FuncCall(
                        String::from("cd"),
                        vec![
                            Variable(String::from("e")),
                            Literal(String::from("fg"))
                        ]
                    ),
                    Literal(String::from("hi"))
                ]
            )
        ]);
    }

   #[test]
    fn test_literal_variable() {
        let parsed = parse("ab").unwrap();
        assert_eq!(parsed, vec![
            Literal(String::from("ab"))
        ]);
    }

   #[test]
    fn test_literal() {
        let parsed = parse("ab").unwrap();
        assert_eq!(parsed, vec![
            Literal(String::from("ab"))
        ]);
    }
}
