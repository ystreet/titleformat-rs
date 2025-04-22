use crate::environment::Value;
use crate::functions::num::to_int;
use crate::types::Error;
use crate::types::Error::*;

/*
 * $ifequal(int1,int2,then,else)
 *
 * Compares the integer numbers int1 and int2, if int1 is equal to int2, the
 * then part is evaluated and its value returned. Otherwise the else part
 * is evaluated and its value returned.
 */
pub fn ifequal(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 4 {
        return Err(InvalidNativeFunctionArgs(
            String::from("ifequal"),
            args.len(),
        ));
    }
    if to_int(&args[0].val) == to_int(&args[1].val) {
        Ok(args[2].clone())
    } else {
        Ok(args[3].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::value_string;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            ifequal(vec![value_string("blah", false)]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("ifequal"), 1)
        );
    }

    #[test]
    fn equal() {
        assert_eq!(
            ifequal(vec![
                value_string("1", false),
                value_string("1", true),
                value_string("true", false),
                value_string("false", false)
            ])
            .unwrap(),
            value_string("true", false)
        );
    }

    #[test]
    fn non_equal() {
        assert_eq!(
            ifequal(vec![
                value_string("1", false),
                value_string("2", true),
                value_string("true", false),
                value_string("false", false)
            ])
            .unwrap(),
            value_string("false", false)
        );
    }
}
