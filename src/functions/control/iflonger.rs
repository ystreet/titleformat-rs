use types::Error;
use types::Error::*;
use environment::Value;
use functions::num::to_int;

/*
 * $iflonger(str,n,then,else)
 *
 * Compares the length of the string str to the number n, if str is longer
 * than n characters, the then part is evaluated and its value returned.
 * Otherwise the else part is evaluated and its value returned.  
 */
pub fn iflonger (args : Vec<Value>) -> Result<Value, Error> {
    if args.len() != 4 {
        return Err(InvalidNativeFunctionArgs(String::from("iflonger"), args.len()));
    }
    if args[0].val.len() > to_int (&args[1].val) as usize {
        Ok(args[2].clone())
    } else {
        Ok(args[3].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use environment::value_string;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(iflonger(
            vec![
                value_string("blah", false)
            ]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("iflonger"), 1));
    }

    #[test]
    fn longer() {
        assert_eq!(iflonger(
            vec![
                value_string("string", false),
                value_string("1", true),
                value_string("true", false),
                value_string("false", false)
            ]).unwrap(),
            value_string("true", false));
    }

    #[test]
    fn shorter() {
        assert_eq!(iflonger(
            vec![
                value_string("string", false),
                value_string("10", true),
                value_string("true", false),
                value_string("false", false)
            ]).unwrap(),
            value_string("false", false));
    }

    #[test]
    fn equal() {
        assert_eq!(iflonger(
            vec![
                value_string("string", false),
                value_string("6", true),
                value_string("true", false),
                value_string("false", false)
            ]).unwrap(),
            value_string("false", false));
    }
}
