use types::Error;
use types::Error::*;
use environment::Value;
use environment::value_string;

/* $not(expr)
 *
 * Logical Not. Returns the logical opposite of EXPR: false, if expr is
 * true and true if expr is false. 
 */
pub fn xor(args : Vec<Value>) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("xor"), args.len()));
    }
    Ok(value_string ("", args.iter().fold(false, |cur, x| { cur ^ x.cond })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(xor(
            vec![
                value_string("", false)
            ]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("xor"), 1));
    }

    #[test]
    fn test_false() {
        assert_eq!(
            xor(vec![
                value_string("", true),
                value_string("", true),
            ]).unwrap(),
            value_string("", false)
        );
    }

    #[test]
    fn test_true() {
        assert_eq!(
            xor(vec![
                value_string("", false),
                value_string("", true),
            ]).unwrap(),
            value_string("", true)
        );
    }

    #[test]
    fn test_multi_true() {
        assert_eq!(
            xor(vec![
                value_string("", true),
                value_string("", true),
                value_string("", true),
            ]).unwrap(),
            value_string("", true)
        );
    }

    #[test]
    fn test_multi_almost_false() {
        assert_eq!(
            xor(vec![
                value_string("", false),
                value_string("", true),
                value_string("", false),
            ]).unwrap(),
            value_string("", true)
        );
    }
}
