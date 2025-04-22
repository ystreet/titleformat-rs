use crate::environment::value_string;
use crate::environment::Value;
use crate::types::Error;
use crate::types::Error::*;

/*
 * $if(cond,then,else)
 *
 * If cond evaluates to true, the then part is evaluated and its value returned.
 * Otherwise, the else part is evaluated and its value returned.
 */
pub fn if_(args: Vec<Value>) -> Result<Value, Error> {
    match args.len() {
        2 | 3 => Ok(()),
        _ => Err(InvalidNativeFunctionArgs(String::from("if"), args.len())),
    }?;
    if args[0].cond {
        Ok(args[1].clone())
    } else if args.len() > 2 {
        Ok(args[2].clone())
    } else {
        Ok(value_string("", false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            if_(vec![value_string("blah", false)]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("if"), 1)
        );
    }

    #[test]
    fn if_false_default() {
        assert_eq!(
            if_(vec![
                value_string("blah", false),
                value_string("true", true)
            ])
            .unwrap(),
            value_string("", false)
        );
    }

    #[test]
    fn if_true_default() {
        assert_eq!(
            if_(vec![value_string("blah", true), value_string("true", true)]).unwrap(),
            value_string("true", true)
        );
    }

    #[test]
    fn if_false_else() {
        assert_eq!(
            if_(vec![
                value_string("blah", false),
                value_string("true", true),
                value_string("false", true),
            ])
            .unwrap(),
            value_string("false", true)
        );
    }

    #[test]
    fn if_true_else() {
        assert_eq!(
            if_(vec![
                value_string("blah", true),
                value_string("true", true),
                value_string("false", true),
            ])
            .unwrap(),
            value_string("true", true)
        );
    }
}
