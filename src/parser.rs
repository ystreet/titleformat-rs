use std::str;

use nom::{alphanumeric, non_empty};
use nom::types::CompleteStr;
use nom::{IResult, ErrorKind};

use types::Expr;
use types::Expr::*;
use types::Error;
use types::Error::*;

named!(varname<CompleteStr, CompleteStr>, ws!(alphanumeric));

/* based on https://wiki.hydrogenaud.io/index.php?title=Foobar2000:Title_Formatting_Reference */
/* TODO: comments, newlines, escaping and conditionals, expand literal */

/* comment
 *
 * A comment is a line starting with two slashes, e.g. // this is a comment. */
named!(comment<CompleteStr, Expr>,
    do_parse!(
        alt!(
            delimited!(tag!("//"), take_until!("\n"), tag!("\n"))
          | delimited!(tag!("//"), take_until!("\r\n"), tag!("\r\n"))
          | preceded!(tag!("//"), take_till!(|_| { false }))
        ) >> 
        (Nil)
    )
);

/* %varname%
 *
 * A field reference is a field name enclosed in percent signs, for example %artist%.
 */
named!(variable<CompleteStr, Expr>,
    delimited!(
        tag!("%"),
        return_error!(ErrorKind::Custom(1),
            do_parse!(
                var_name : literal >>
                (parse_varname(&var_name))
            )
        ),
        tag!("%")
    )
);

/* $funcname(arg1,arg2)
 *
 * A function call starts with a dollar sign, followed by the function name
 * and the parameter list. A parameter list can either be empty—denoted as
 * ()—or contain one or more parameters separated by commas, for example
 * $abbr(%artist%). A parameter can be literal text, a field reference, or
 * another function call. Note that there must be no whitespace between the
 * dollar sign and the function name, or the function name and the opening
 * parenthesis of the parameter list. 
 */
named!(func<CompleteStr, Expr>,
    preceded!(tag!("$"),
        return_error!(ErrorKind::Custom(2),
            do_parse!(
                func_name: varname >>
                args : ws!(delimited!(char!('('), separated_list!(char!(','), nested_expr), char!(')'))) >>
                (parse_funccall(&func_name, args))
            )
        )
    )
);

/* text literal
 *
 * Any other text is literal text. In literal text, the character
 * %, $, [, ], or ' (apostrophe/single quote) must be escaped by enclosing it
 * in ' (apostrophe/single quote) characters. For example, '[' (a left bracket
 * in single quotes) results in a literal [ (left bracket). As a special case,
 * '' (two single quotes in a row) results in one single quote. In the
 * playlist, < and > are also special; see #Dimmed and highlighted text.
 */
fn is_special(c : char) -> bool {
  /* anything that may not be a literal */
  match c {
    '%' | '$' | ',' | '[' | ']' | '<' | '>' | '\'' | '(' | ')' | '/' | '\r' | '\n'  => true,
    _ => false
  }
}

named!(literal<CompleteStr, CompleteStr>,
    alt!(
        take_till1!(is_special)
      | map!(tag!("\'%\'"), |_| CompleteStr("%"))
      | map!(tag!("\'$\'"), |_| CompleteStr("$"))
      | map!(tag!("\'[\'"), |_| CompleteStr("["))
      | map!(tag!("\']\'"), |_| CompleteStr("]"))
      | map!(tag!("\'\'"),  |_| CompleteStr("\'"))
      | map!(tag!("/"),     |_| CompleteStr("/"))
      | map!(tag!("\n"),    |_| CompleteStr(""))
      | map!(tag!("\r\n"),  |_| CompleteStr(""))
));
named!(literal_expr<CompleteStr, Expr>,
    do_parse!(
        lit : literal >>
        (parse_literal(&lit))
    )
);

named!(nested_expr<CompleteStr, Expr>, alt!(comment | func | variable | literal_expr));
named!(expr<CompleteStr, Vec<Expr>>, many0!(nested_expr));

fn simplify (input : Vec<Expr>) -> Vec<Expr> {
    /* remove Nil's, empty literal's and consecutive literals */
    let mut iter = input.iter().peekable();
    let mut new = Vec::new();

    /* loop and .next() to get around borrowing with for a in iterator */
    loop {
        let expr = iter.next();
        match expr {
            None => break,
            Some(Nil) => {},
            Some(Literal(v)) => {
                let mut cloned = v.clone();
                /* literal, nil, ?? -> literal, opt(??) */
                println!("dealing with literal {}: {:?}", v, input);
                if let Some(Nil) = iter.peek() {
                    iter.next();
                    println!("got nil after literal {}: {:?}", v, input);
                    match iter.peek() {
                        None => {
                            /* push the value, no further interesting values */
                            new.push(Literal(cloned));
                        },
                        Some(Literal(v2)) => {
                            /* literal, nil, literal -> combined literal */
                            cloned.push_str(&v2);
                            new.push(Literal(cloned));
                        }
                        _ => new.push(Literal(cloned))
                    }
                } else {
                    new.push(Literal(cloned))
                }
            }
            Some(Variable(var)) => new.push(Variable(var.clone())),
//            Conditional(c) => new.push(Conditional(c.clone())), /* FIXME: */
            Some(FuncCall(name, args)) => new.push(FuncCall(name.clone(), simplify(args.to_vec()))),
        }
    }
    new
}

pub fn parse(input: &str) -> Result<Vec<Expr>, Error> {
    match expr(CompleteStr(input)) {
        Ok((_, expr)) => {println!("{:?}", expr); Ok(simplify(expr))},
        e => { println!("{:?}", e); Err(ParseError)},
    }
}

fn parse_literal(literal: &str) -> Expr {
    println!("got literal {}", literal);
    Literal(String::from(literal))
}

fn parse_varname(name: &str) -> Expr {
    println!("got variable {}", name);
    Variable(String::from(name))
}

fn parse_funccall(name: &str, args : Vec<Expr>) -> Expr {
    println!("got function {}", name);
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
        let parsed = parse("ab -_=+").unwrap();
        assert_eq!(parsed, vec![
            Literal(String::from("ab -_=+"))
        ]);
    }

   #[test]
    fn test_escaped_literal() {
        assert_eq!(parse("\'%\'").unwrap(), vec![
            Literal(String::from("%"))
        ]);
        assert_eq!(parse("\'$\'").unwrap(), vec![
            Literal(String::from("$"))
        ]);
        assert_eq!(parse("\'[\'").unwrap(), vec![
            Literal(String::from("["))
        ]);
        assert_eq!(parse("\']\'").unwrap(), vec![
            Literal(String::from("]"))
        ]);
        assert_eq!(parse("\'\'").unwrap(), vec![
            Literal(String::from("\'"))
        ]);
    }

   #[test]
    fn test_function_newlines() {
        /* all newlines between tokens are ignored */
        assert_eq!(parse("$f(var)\n").unwrap(), vec![
            FuncCall(String::from("f"), vec![
                Literal(String::from("var"))
            ])
        ]);
        assert_eq!(parse("$f(\nvar)").unwrap(), vec![
            FuncCall(String::from("f"), vec![
                Literal(String::from("var"))
            ])
        ]);
        assert_eq!(parse("$f(\nvar\n)").unwrap(), vec![
            FuncCall(String::from("f"), vec![
                Literal(String::from("var"))
            ])
        ]);
    }
/*
   #[test]
    fn test_function_comment() {
        assert_eq!(parse("$f(var//\n)").unwrap(), vec![
            FuncCall(String::from("f"), vec![
                Literal(String::from("var"))
            ])
        ]);
    }
*/
   #[test]
    fn test_empty_comment() {
        assert_eq!(parse("//\n").unwrap(), vec![]);
    }

   #[test]
    fn test_comment() {
        let mut parsed = parse("// comment\n").unwrap();
        assert_eq!(parsed, vec![]);
        parsed = parse("// comment\r\n").unwrap();
        assert_eq!(parsed, vec![]);
        parsed = parse("// comment").unwrap();
        assert_eq!(parsed, vec![]);
    }
}
