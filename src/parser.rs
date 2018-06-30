use std::str;

use nom::types::CompleteStr;
use nom::ErrorKind;

use types::Expr;
use types::Expr::*;
use types::Error;
use types::Error::*;

/* based on https://wiki.hydrogenaud.io/index.php?title=Foobar2000:Title_Formatting_Reference */

/* comment
 *
 * A comment is a line starting with two slashes, e.g. // this is a comment. */
named!(comment<CompleteStr, String>,
    do_parse!(
        alt!(
            delimited!(tag!("//"), take_until!("\n"), tag!("\n"))
          | delimited!(tag!("//"), take_until!("\r\n"), tag!("\r\n"))
          | preceded!(tag!("//"), take_till!(|_| { false }))
        ) >> 
        (String::from(""))
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
                var_name : take_until1!("%") >>
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
                func_name: take_until1!("(") >>
                args : ws!(delimited!(char!('('), separated_list!(char!(','), function_expr), char!(')'))) >>
                (parse_funccall(&func_name, args))
            )
        )
    )
);

/* Evaluates the expression between [ and ]. If it has the truth value true,
 * its string value and the truth value true are returned. Otherwise an empty
 * string and false are returned.

 * Example: [%artist%] returns the value of the artist tag, if it exists.
 * Otherwise it returns nothing, when artist would return "?". 
 */
named!(conditional<CompleteStr, Expr>,
    delimited!(
        tag!("["),
        return_error!(ErrorKind::Custom(3),
            do_parse!(
                expr : conditional_expr >>
                (parse_conditional(expr))
            )
        ),
        tag!("]")
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

named!(unescaped_literal<CompleteStr, String>,
    map!(take_till1!(is_special), |a| String::from(&a as &str))
);

named!(escaped_literal<CompleteStr, String>,
    do_parse!(
        tag!("\'") >>
        ret : take_until1!("\'") >>
        tag!("\'") >>
        (String::from(&ret as &str))
    )
);

/* literal that can be detected anywhere */
named!(base_literal<CompleteStr, String>,
    do_parse!(
        opt!(comment) >>
        ret : alt!(
            unescaped_literal
          | escaped_literal
            /* [, ], <, > currently unused */
          | map!(tag!("<"),     |_| String::from("<"))
          | map!(tag!(">"),     |_| String::from(">"))
          | map!(tag!("\'\'"),  |_| String::from("\'"))
          | map!(tag!("/"),     |_| String::from("/"))
          | map!(tag!("\n"),    |_| String::from(""))
          | map!(tag!("\r\n"),  |_| String::from(""))) >>
        opt!(comment) >>
        (ret)
    )
);

named!(function_literal<CompleteStr, String>,
    alt!(
        base_literal
      | map!(tag!("("),  |_| String::from("("))
      | map!(tag!("]"),  |_| String::from("]"))
    )
);
named!(function_literal_expr<CompleteStr, Expr>,
    do_parse!(
        lit : fold_many1!(function_literal,
            String::new(), |mut acc : String, item : String| {
                acc.push_str(&item);
                acc
            }) >>
        (parse_literal(&lit))
    )
);
named!(function_expr<CompleteStr, Expr>, alt!(func | variable | function_literal_expr));

named!(conditional_literal<CompleteStr, String>,
    alt!(
        base_literal
      | map!(tag!("("),  |_| String::from("("))
      | map!(tag!(")"),  |_| String::from(")"))
      | map!(tag!(","),  |_| String::from(","))
    )
);
named!(conditional_literal_expr<CompleteStr, Expr>,
    do_parse!(
        lit : fold_many1!(conditional_literal,
            String::new(), |mut acc : String, item : String| {
                acc.push_str(&item);
                acc
            }) >>
        (parse_literal(&lit))
    )
);
named!(conditional_expr<CompleteStr, Expr>, alt!(func | variable | conditional_literal_expr));

/* literals outside functions, variables and conditionas */
named!(standard_literal<CompleteStr, String>,
    alt!(
        base_literal
      | map!(tag!("("),  |_| String::from("("))
      | map!(tag!(")"),  |_| String::from(")"))
      | map!(tag!("]"),  |_| String::from("]"))
      | map!(tag!(","),  |_| String::from(","))
    )
);
named!(standard_literal_expr<CompleteStr, Expr>,
    do_parse!(
        lit : fold_many1!(standard_literal,
            String::new(), |mut acc : String, item : String| {
                acc.push_str(&item);
                acc
            }) >>
        (parse_literal(&lit))
    )
);

named!(nested_expr<CompleteStr, Expr>, alt!(conditional | func | variable | standard_literal_expr));
named!(expr<CompleteStr, Vec<Expr>>, many0!(nested_expr));

pub fn parse(input: &str) -> Result<Vec<Expr>, Error> {
    match expr(CompleteStr(input)) {
        Ok((_, expr)) => {println!("{:?}", expr); Ok(expr)},
        e => { println!("{:?}", e); Err(ParseError)},
    }
}

fn parse_conditional(conditional : Expr) -> Expr {
    println!("got conditional {:?}", conditional);
    Conditional(Box::new(conditional))
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
        assert_eq!(parse("$f(\nvar\r\n)").unwrap(), vec![
            FuncCall(String::from("f"), vec![
                Literal(String::from("var"))
            ])
        ]);
    }

   #[test]
    fn test_function_comment() {
        assert_eq!(parse("$f(var//\n)").unwrap(), vec![
            FuncCall(String::from("f"), vec![
                Literal(String::from("var"))
            ])
        ]);
    }

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

   #[test]
    fn test_possibly_special_literals() {
        let mut parsed = parse(",").unwrap();
        assert_eq!(parsed, vec![Literal(String::from(","))]);
        parsed = parse("<").unwrap();
        assert_eq!(parsed, vec![Literal(String::from("<"))]);
        parsed = parse(">").unwrap();
        assert_eq!(parsed, vec![Literal(String::from(">"))]);
        parsed = parse("(").unwrap();
        assert_eq!(parsed, vec![Literal(String::from("("))]);
        parsed = parse(")").unwrap();
        assert_eq!(parsed, vec![Literal(String::from(")"))]);
        parsed = parse("/").unwrap();
        assert_eq!(parsed, vec![Literal(String::from("/"))]);
    }

   #[test]
    fn test_combined_special() {
        /* tests that special characters in literals are combined correctly */
        let parsed = parse("a,b,c(d]").unwrap();
        assert_eq!(parsed, vec![Literal(String::from("a,b,c(d]"))]);
    }

   #[test]
    fn test_conditional_special() {
        let parsed = parse("[a),(]").unwrap();
        assert_eq!(parsed, vec![Conditional(Box::new(Literal(String::from("a),("))))]);
    }

   #[test]
    fn test_function_special() {
        let parsed = parse("$a(b(])").unwrap();
        assert_eq!(parsed, vec![FuncCall(String::from("a"),
            vec![Literal(String::from("b(]"))])]);
    }

   #[test]
    fn test_conditional_literal() {
        let parsed = parse("[a]").unwrap();
        assert_eq!(parsed, vec![Conditional(Box::new(Literal(String::from("a"))))]);
    }

   #[test]
    fn test_conditional_variable() {
        let parsed = parse("[%a%]").unwrap();
        assert_eq!(parsed, vec![Conditional(Box::new(Variable(String::from("a"))))]);
    }

   #[test]
    fn test_conditional_function() {
        let parsed = parse("[$a(b)]").unwrap();
        assert_eq!(parsed, vec![Conditional(Box::new(FuncCall(
            String::from("a"),
            vec![Literal(String::from("b"))]
        )))]);
    }
}
