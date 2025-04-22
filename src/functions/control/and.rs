use crate::environment::value_string;
use crate::environment::Value;
use crate::types::Error;
use crate::types::Error::*;

/* $and(expr, ...)
 *
 * Logical And of an arbitrary number of arguments. Returns true, if and
 * only if all expr arguments evaluate to true.
 */
pub fn and(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("and"), args.len()));
    }
    Ok(value_string("", args.iter().all(|x| x.cond)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            and(vec![value_string("", false)]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("and"), 1)
        );
    }

    #[test]
    fn test_true() {
        assert_eq!(
            and(vec![value_string("", true), value_string("", true)]).unwrap(),
            value_string("", true)
        );
    }

    #[test]
    fn test_false() {
        assert_eq!(
            and(vec![value_string("", false), value_string("", true),]).unwrap(),
            value_string("", false)
        );
    }
}
