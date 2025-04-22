use crate::environment::value_string;
use crate::environment::Value;
use crate::functions::num::to_int;
use crate::types::Error;
use crate::types::Error::*;

/*
 * $select(n,a1,...,aN)
 *
 * If the value of n is between 1 and N, aN is evaluated and its value
 * returned. Otherwise false is returned.
 */
pub fn select(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(
            String::from("select"),
            args.len(),
        ));
    };
    let mut iter = args.iter();
    let n = to_int(&iter.next().unwrap().val) as usize;
    if n > 0 && n < args.len() {
        Ok(args[n].clone())
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
            select(vec![value_string("blah", false)]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("select"), 1)
        );
    }

    #[test]
    fn out_of_bounds() {
        assert_eq!(
            select(vec![value_string("3", false), value_string("true", true)]).unwrap(),
            value_string("", false)
        );
    }

    #[test]
    fn select_valid() {
        assert_eq!(
            select(vec![
                value_string("2", false),
                value_string("1", false),
                value_string("2", false),
                value_string("3", false),
                value_string("4", true)
            ])
            .unwrap(),
            value_string("2", false)
        );
    }
}
