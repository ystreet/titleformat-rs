use types::Error;
use types::Error::*;
use environment::Value;

/*
 * $if3(a1,a2,...,aN,else)
 *
 * Evaluates arguments a1 ... aN, until one is found that evaluates to true.
 * If that happens, its value is returned. Otherwise the else part is
 * evaluated and its value returned. 
 */
pub fn if3 (args : Vec<Value>) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("if3"), args.len()));
    };
    for v in &args {
        if v.cond {
            return Ok(v.clone())
        }
    }
    Ok(args[args.len()-1].clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use environment::value_string;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(if3(
            vec![
                value_string("blah", false)
            ]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("if3"), 1));
    }

    #[test]
    fn if3_2() {
        assert_eq!(if3(
            vec![
                value_string("blah", false),
                value_string("true", true)
            ]).unwrap(),
            value_string("true", true));
    }

    #[test]
    fn if3_5() {
        assert_eq!(if3(
            vec![
                value_string("blah", false),
                value_string("blah", false),
                value_string("blah", false),
                value_string("blah", false),
                value_string("true", true)
            ]).unwrap(),
            value_string("true", true));
    }

    #[test]
    fn if3_else() {
        assert_eq!(if3(
            vec![
                value_string("blah", false),
                value_string("blah", false),
                value_string("blah", false),
                value_string("blah", false),
                value_string("true", false)
            ]).unwrap(),
            value_string("true", false));
    }
}
