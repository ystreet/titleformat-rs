use crate::environment::value_string;
use crate::environment::Value;
use crate::types::Error;
use crate::types::Error::*;

/* $not(expr)
 *
 * Logical Not. Returns the logical opposite of EXPR: false, if expr is
 * true and true if expr is false.
 */
pub fn not(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(InvalidNativeFunctionArgs(String::from("not"), args.len()));
    }
    Ok(value_string("", !args[0].cond))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            not(vec![value_string("", false), value_string("", false)])
                .err()
                .unwrap(),
            InvalidNativeFunctionArgs(String::from("not"), 2)
        );
    }

    #[test]
    fn test_true() {
        assert_eq!(
            not(vec![value_string("", true),]).unwrap(),
            value_string("", false)
        );
    }

    #[test]
    fn test_false() {
        assert_eq!(
            not(vec![value_string("", false),]).unwrap(),
            value_string("", true)
        );
    }
}
