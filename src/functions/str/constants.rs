use crate::environment::value_string;
use crate::environment::Value;
use crate::functions::num::to_int;
use crate::types::Error;
use crate::types::Error::*;

/*
 * $crlf()
 * Inserts end-of-line marker (carriage return, line feed). Can be used to
 * generate multiple lines in the output, for example for the tooltip of
 * the system notification area ("systray") icon.
 */
pub fn crlf(args: Vec<Value>) -> Result<Value, Error> {
    match args.len() {
        0 => Ok(value_string("\r\n", true)),
        _ => Err(InvalidNativeFunctionArgs(String::from("crlf"), args.len())),
    }
}

pub fn tab(args: Vec<Value>) -> Result<Value, Error> {
    match args.len() {
        0 => Ok(value_string("\t", true)),
        1 => {
            let count = to_int(&args[0].val);
            if count > 256 {
                Err(Error::OutOfRange)
            } else {
                Ok(Value {
                    val: (0..count).fold(String::from(""), |mut acc, _i| {
                        acc.push('\t');
                        acc
                    }),
                    cond: true,
                })
            }
        }
        _ => Err(InvalidNativeFunctionArgs(String::from("tab"), args.len())),
    }
}

pub fn noop(_args: Vec<Value>) -> Result<Value, Error> {
    Ok(value_string("", true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            crlf(vec![value_string("blah", false)]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("crlf"), 1)
        );
        assert_eq!(
            tab(vec![
                value_string("blah", false),
                value_string("blah", false)
            ])
            .err()
            .unwrap(),
            InvalidNativeFunctionArgs(String::from("tab"), 2)
        );
    }

    #[test]
    fn test_crlf() {
        assert_eq!(crlf(vec![]).unwrap(), value_string("\r\n", true));
    }

    #[test]
    fn test_tab() {
        assert_eq!(tab(vec![]).unwrap(), value_string("\t", true));
        assert_eq!(
            tab(vec![value_string("4", true)]).unwrap(),
            value_string("\t\t\t\t", true)
        );
    }

    #[test]
    fn test_noop() {
        assert_eq!(noop(vec![]).unwrap(), value_string("", true));
        assert_eq!(
            noop(vec![value_string("jibberish", true)]).unwrap(),
            value_string("", true)
        );
    }
}
