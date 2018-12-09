use types::Error;
use types::Error::*;
use environment::Value;
use environment::value_string;

pub fn len(args : Vec<Value>) -> Result<Value, Error> {
    match args.len() {
        1 => {
          let val = &args[0];
          Ok(value_string (val.val.len().to_string().as_str(), val.cond))
        },
        _ => Err(InvalidNativeFunctionArgs(String::from("len"), args.len())),
    }
}

pub fn longer(args : Vec<Value>) -> Result<Value, Error> {
    match args.len() {
        2 => {
          Ok(value_string ("", args[0].val.len() > args[1].val.len()))
        },
        _ => Err(InvalidNativeFunctionArgs(String::from("longer"), args.len())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(len(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("len"), 0));
        assert_eq!(longer(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("longer"), 0));
    }

    #[test]
    fn test_len() {
        assert_eq!(len(vec![value_string ("12345", true)]).unwrap(),
            value_string("5", true));
    }

    #[test]
    fn test_longer() {
        /* do we return the longer string as well? */
        assert_eq!(longer(vec![value_string ("12345", true), value_string ("123", true)]).unwrap(),
            value_string("", true));
        assert_eq!(longer(vec![value_string ("123", true), value_string ("12345", true)]).unwrap(),
            value_string("", false));
    }
}
