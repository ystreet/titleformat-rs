use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_till1, take_until};
use nom::character::complete::newline;
use nom::combinator::{all_consuming, map, opt};
use nom::multi::{fold_many0, fold_many1, many0, separated_list0};
use nom::sequence::{delimited, preceded};
use nom::{IResult, Input, Parser};

use crate::types::Error;
use crate::types::Error::*;
use crate::types::Expr;
use crate::types::Expr::*;

/* based on https://wiki.hydrogenaud.io/index.php?title=Foobar2000:Title_Formatting_Reference */

/* comment
 *
 * A comment is a line starting with two slashes, e.g. // this is a comment. */
fn comment(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(tag("//"), take_until("\n"), tag("\n")),
        delimited(tag("//"), take_until("\r\n"), tag("\r\n")),
        preceded(tag("//"), take_till(|_| false)),
    ))
    .parse(input)
}

fn newlines(input: &str) -> IResult<&str, &str> {
    alt((map(tag("\n"), |_| ""), map(tag("\r\n"), |_| ""))).parse(input)
}

/* %varname%
 *
 * A field reference is a field name enclosed in percent signs, for example %artist%.
 */
fn variable(input: &str) -> IResult<&str, Expr> {
    delimited(tag("%"), take_until("%"), tag("%"))
        .parse(input)
        .map(|(input, var)| (input, parse_varname(var)))
}

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
fn func2(input: &str) -> IResult<&str, Expr> {
    let (input, func_name) = take_until("(")(input)?;
    let (input, _) = many0(newline).parse(input)?;
    let (input, args) = func_args(input)?;
    let (input, _) = many0(newline).parse(input)?;
    Ok((input, parse_funccall(func_name, args)))
}

fn func(input: &str) -> IResult<&str, Expr> {
    preceded(tag("$"), func2).parse(input)
}

fn find_func_arg_end(input: &str) -> IResult<&str, &str> {
    let mut stack = 1;
    for (index, c) in input.iter_indices() {
        match c {
            '$' => stack += 1,
            ')' => {
                stack -= 1;
                if stack == 0 {
                    return Ok((input.take_from(index), input.take(index)));
                }
            }
            ',' => {
                if stack == 1 {
                    return Ok((input.take_from(index), input.take(index)));
                }
            }
            _ => (),
        }
    }
    Err(nom::Err::Failure(nom::error::Error::new(
        input,
        nom::error::ErrorKind::SeparatedList,
    )))
}
fn func_args(input: &str) -> IResult<&str, Vec<Vec<Expr>>> {
    let (input, args) = delimited(
        tag("("),
        separated_list0(tag(","), find_func_arg_end),
        tag(")"),
    )
    .parse(input)?;
    let mut ret = vec![];
    for expr in args {
        ret.push(function_expr(expr)?.1);
    }
    Ok((input, ret))
}

/* Evaluates the expression between [ and ]. If it has the truth value true,
* its string value and the truth value true are returned. Otherwise an empty
* string and false are returned.

* Example: [%artist%] returns the value of the artist tag, if it exists.
* Otherwise it returns nothing, when artist would return "?".
*/
fn find_conditional_end(input: &str) -> IResult<&str, &str> {
    let mut stack = 1;
    for (index, c) in input.iter_indices() {
        match c {
            '[' => stack += 1,
            ']' => {
                stack -= 1;
                if stack == 0 {
                    return Ok((input.take_from(index), input.take(index)));
                }
            }
            _ => (),
        }
    }
    Err(nom::Err::Failure(nom::error::Error::new(
        input,
        nom::error::ErrorKind::SeparatedList,
    )))
}
fn conditional(input: &str) -> IResult<&str, Expr> {
    let (del_input, cond_expr) =
        delimited(tag("["), find_conditional_end, tag("]")).parse(input)?;
    let (cond_input, expr) = conditional_expr(cond_expr).map(|(_, expr)| (del_input, expr))?;
    if !cond_input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Eof,
        }));
    }
    Ok((del_input, expr))
}

/* text literal
 *
 * Any other text is literal text. In literal text, the character
 * %, $, [, ], or ' (apostrophe/single quote) must be escaped by enclosing it
 * in ' (apostrophe/single quote) characters. For example, '[' (a left bracket
 * in single quotes) results in a literal [ (left bracket). As a special case,
 * '' (two single quotes in a row) results in one single quote. In the
 * playlist, < and > are also special; see #Dimmed and highlighted text.
 */
fn is_special(c: char) -> bool {
    /* anything that may not be a literal */
    matches!(
        c,
        '%' | '$' | ',' | '[' | ']' | '<' | '>' | '\'' | '(' | ')' | '/' | '\r' | '\n'
    )
}

fn unescaped_literal(input: &str) -> IResult<&str, &str> {
    take_till1(is_special).parse(input)
}

fn escaped_literal(input: &str) -> IResult<&str, &str> {
    delimited(tag("\'"), take_until("\'"), tag("\'")).parse(input)
}

/* literal that can be detected anywhere */
fn base_literal(input: &str) -> IResult<&str, &str> {
    let (input, _comment_val) = opt(comment).parse(input)?;
    if input.is_empty() {
        //&& comment.is_some() {
        //return Ok((input, ""));
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Complete,
        )));
    }
    let (input, literal) = alt((
        map(tag("\'\'"), |_| "\'"),
        unescaped_literal,
        escaped_literal,
        newlines,
        tag("<"),
        tag(">"),
        tag("/"),
    ))
    .parse(input)?;
    let (input, _comment_val) = opt(comment).parse(input)?;
    Ok((input, literal))
}

fn function_literal(input: &str) -> IResult<&str, &str> {
    alt((tag("("), tag("]"), base_literal)).parse(input)
}

fn function_literal_expr(input: &str) -> IResult<&str, Expr> {
    fold_many1(
        function_literal,
        String::new,
        |mut acc: String, item: &str| {
            acc.push_str(item);
            acc
        },
    )
    .map(|lit| parse_literal(&lit))
    .parse(input)
}
fn function_expr(input: &str) -> IResult<&str, Vec<Expr>> {
    all_consuming(many0(alt((
        conditional,
        func,
        variable,
        function_literal_expr,
    ))))
    .parse(input)
}

fn conditional_literal(input: &str) -> IResult<&str, &str> {
    alt((tag(")"), tag("("), tag(","), base_literal)).parse(input)
}

fn conditional_literal_expr(input: &str) -> IResult<&str, Expr> {
    fold_many1(
        conditional_literal,
        String::new,
        |mut acc: String, item: &str| {
            acc.push_str(item);
            acc
        },
    )
    .map(|lit| parse_literal(&lit))
    .parse(input)
}
fn conditional_expr(input: &str) -> IResult<&str, Expr> {
    all_consuming(many0(alt((
        conditional,
        func,
        variable,
        conditional_literal_expr,
    ))))
    .parse(input)
    .map(|(input, conditional)| (input, parse_conditional(conditional)))
}

/* literals outside functions, variables and conditionas */
fn standard_literal(input: &str) -> IResult<&str, &str> {
    alt((tag("("), tag(")"), tag("]"), tag(","), base_literal)).parse(input)
}
fn standard_literal_expr(input: &str) -> IResult<&str, Expr> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Complete,
        )));
    }
    fold_many1(
        standard_literal,
        String::new,
        |mut acc: String, item: &str| {
            acc.push_str(item);
            acc
        },
    )
    .map(|lit| parse_literal(&lit))
    .parse(input)
}

fn nested_expr(input: &str) -> IResult<&str, Expr> {
    alt((conditional, func, variable, standard_literal_expr)).parse(input)
}
fn expr(input: &str) -> IResult<&str, Vec<Expr>> {
    fold_many0(nested_expr, Vec::new, move |mut acc, item| {
        acc.push(item);
        acc
    })
    .parse(input)
}

pub fn parse(input: &str) -> Result<Vec<Expr>, Error> {
    match expr(input) {
        Ok((_, expr)) => Ok(expr),
        _e => Err(ParseError),
    }
}

fn parse_conditional(conditional: Vec<Expr>) -> Expr {
    Conditional(conditional)
}

fn parse_literal(literal: &str) -> Expr {
    Literal(String::from(literal))
}

fn parse_varname(name: &str) -> Expr {
    Variable(String::from(name))
}

fn parse_funccall(name: &str, args: Vec<Vec<Expr>>) -> Expr {
    FuncCall(String::from(name), args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let parsed = parse("").unwrap();
        assert_eq!(parsed, vec![]);
    }

    #[test]
    fn test_variable() {
        let parsed = parse("%ab%").unwrap();
        assert_eq!(parsed, vec![Variable(String::from("ab"))]);
    }

    #[test]
    fn test_empty_funccall() {
        let parsed = parse("$ab()").unwrap();
        assert_eq!(parsed, vec![FuncCall(String::from("ab"), vec![vec![]])]);
    }

    #[test]
    fn test_funccall_variable() {
        let parsed = parse("$ab(%ba%)").unwrap();
        assert_eq!(
            parsed,
            vec![FuncCall(
                String::from("ab"),
                vec![vec![Variable(String::from("ba"))]]
            )]
        );
    }

    #[test]
    fn test_funccall_funccall() {
        let parsed = parse("$ab($ba())").unwrap();
        assert_eq!(
            parsed,
            vec![FuncCall(
                String::from("ab"),
                vec![vec![FuncCall(String::from("ba"), vec![vec![]])]]
            )]
        );
    }

    #[test]
    fn test_funccall_complex() {
        let parsed = parse("$ab($cd(%e%,fg),hi)").unwrap();
        assert_eq!(
            parsed,
            vec![FuncCall(
                String::from("ab"),
                vec![
                    vec![FuncCall(
                        String::from("cd"),
                        vec![
                            vec![Variable(String::from("e"))],
                            vec![Literal(String::from("fg"))]
                        ]
                    )],
                    vec![Literal(String::from("hi"))]
                ]
            )]
        );
    }

    #[test]
    fn test_literal_variable() {
        let parsed = parse("ab").unwrap();
        assert_eq!(parsed, vec![Literal(String::from("ab"))]);
    }

    #[test]
    fn test_literal() {
        let parsed = parse("ab -_=+").unwrap();
        assert_eq!(parsed, vec![Literal(String::from("ab -_=+"))]);
    }

    #[test]
    fn test_escaped_literals() {
        assert_eq!(parse("\'%\'").unwrap(), vec![Literal(String::from("%"))]);
        assert_eq!(parse("\'$\'").unwrap(), vec![Literal(String::from("$"))]);
        assert_eq!(parse("\'[\'").unwrap(), vec![Literal(String::from("["))]);
        assert_eq!(parse("\']\'").unwrap(), vec![Literal(String::from("]"))]);
        assert_eq!(parse("\'\'").unwrap(), vec![Literal(String::from("\'"))]);
    }

    #[test]
    fn test_function_newlines() {
        /* all newlines between tokens are ignored */
        assert_eq!(
            parse("$f(var)\n").unwrap(),
            vec![FuncCall(
                String::from("f"),
                vec![vec![Literal(String::from("var"))]]
            )]
        );
        assert_eq!(
            parse("$f(\nvar)").unwrap(),
            vec![FuncCall(
                String::from("f"),
                vec![vec![Literal(String::from("var"))]]
            )]
        );
        assert_eq!(
            parse("$f(\nvar\r\n)").unwrap(),
            vec![FuncCall(
                String::from("f"),
                vec![vec![Literal(String::from("var"))]]
            )]
        );
    }

    #[test]
    fn test_function_comment() {
        assert_eq!(
            parse("$f(var//\n)").unwrap(),
            vec![FuncCall(
                String::from("f"),
                vec![vec![Literal(String::from("var"))]]
            )]
        );
    }

    #[test]
    fn test_unclosed_function() {
        assert_eq!(parse("$f(var"), Err(ParseError),);
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
        assert_eq!(
            parsed,
            vec![Conditional(vec![Literal(String::from("a),("))])]
        );
    }

    #[test]
    fn test_function_special() {
        let parsed = parse("$a(b(])").unwrap();
        assert_eq!(
            parsed,
            vec![FuncCall(
                String::from("a"),
                vec![vec![Literal(String::from("b(]"))]]
            )]
        );
    }

    #[test]
    fn test_conditional_literal() {
        let parsed = parse("[a]").unwrap();
        assert_eq!(parsed, vec![Conditional(vec![Literal(String::from("a"))])]);
    }

    #[test]
    fn test_conditional_variable() {
        let parsed = parse("[%a%]").unwrap();
        assert_eq!(parsed, vec![Conditional(vec![Variable(String::from("a"))])]);
    }

    #[test]
    fn test_conditional_variable_literal() {
        let parsed = parse("[%a%b]").unwrap();
        assert_eq!(
            parsed,
            vec![Conditional(vec![
                Variable(String::from("a")),
                Literal(String::from("b")),
            ])]
        );
    }

    #[test]
    fn test_conditional_function() {
        let parsed = parse("[$a(b)]").unwrap();
        assert_eq!(
            parsed,
            vec![Conditional(vec![FuncCall(
                String::from("a"),
                vec![vec![Literal(String::from("b"))]]
            )])]
        );
    }

    #[test]
    fn test_conditional_conditional() {
        let parsed = parse("[[%a%]]").unwrap();
        assert_eq!(
            parsed,
            vec![Conditional(vec![Conditional(vec![Variable(
                String::from("a")
            )])])]
        );
    }

    #[test]
    fn test_func_conditional() {
        let parsed = parse("$a([%b%])").unwrap();
        assert_eq!(
            parsed,
            vec![FuncCall(
                String::from("a"),
                vec![vec![Conditional(vec![Variable(String::from("b"))])]]
            )]
        );
    }

    #[test]
    fn test_func_empty_arg() {
        let parsed = parse(r"$a(,)").unwrap();
        assert_eq!(
            parsed,
            vec![FuncCall(String::from("a"), vec![vec![], vec![],])]
        );
    }
}
