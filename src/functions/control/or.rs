use crate::environment::value_string;
use crate::environment::Value;
use crate::types::Error;
use crate::types::Error::*;

/* $or(expr, ...)
 *
 * Logical Or of an arbitrary number of arguments. Returns true, if at
 * least one expression evaluates to true.
 */
pub fn or(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("or"), args.len()));
    }
    Ok(value_string("", args.iter().any(|x| x.cond)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            or(vec![value_string("", false)]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("or"), 1)
        );
    }

    #[test]
    fn test_true() {
        assert_eq!(
            or(vec![value_string("", true), value_string("", false),]).unwrap(),
            value_string("", true)
        );
        assert_eq!(
            or(vec![value_string("", false), value_string("", true),]).unwrap(),
            value_string("", true)
        );
    }

    #[test]
    fn test_false() {
        assert_eq!(
            or(vec![value_string("", false), value_string("", false),]).unwrap(),
            value_string("", false)
        );
    }
}
