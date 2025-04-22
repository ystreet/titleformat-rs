use crate::environment::Value;
use crate::types::Error;
use crate::types::Error::*;

/*
 * $if2(expr,else)
 *
 * Like $if(expr,expr,else) except that expr is only evaluated once.
 * In other words, if expression expr is true, expr is returned, otherwise
 * the else part is evaluated and expr is returned as true.
 */
pub fn if2(args: Vec<Value>) -> Result<Value, Error> {
    match args.len() {
        2 => Ok(()),
        _ => Err(InvalidNativeFunctionArgs(String::from("if2"), args.len())),
    }?;
    if args[0].cond {
        Ok(args[0].clone())
    } else {
        Ok(args[1].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::value_string;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            if2(vec![value_string("blah", false)]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("if2"), 1)
        );
    }

    #[test]
    fn if2_false() {
        assert_eq!(
            if2(vec![
                value_string("blah", false),
                value_string("true", true)
            ])
            .unwrap(),
            value_string("true", true)
        );
    }

    #[test]
    fn if2_true() {
        assert_eq!(
            if2(vec![value_string("blah", true), value_string("true", true)]).unwrap(),
            value_string("blah", true)
        );
    }
}
