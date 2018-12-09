use types::Error;
use types::Error::*;
use environment::Value;
use environment::value_string;
use functions::num::to_int;

pub fn num(args : Vec<Value>) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(InvalidNativeFunctionArgs(String::from("num"), args.len()));
    }

    let val = to_int(&args[0].val);
    let len = to_int(&args[1].val);
    let mut s = String::from("");
    if len < 1 || (val < 0 && len < 2)  {
        s.push_str(&val.to_string());
    } else {
        let pos = if val < 0 { -val } else { val };
        let val_s = pos.to_string();
        if val < 0 {
            s.push_str("-");
        }
        while val_s.len() + s.len() < len as usize {
            s.push_str("0");
        }
        s.push_str(&val_s);
    }
    Ok(value_string(&s, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(num(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("num"), 0));
    }

    #[test]
    fn test_num() {
        assert_eq!(num(vec![value_string("-4.8", true), value_string("5", true)]).unwrap(),
            value_string("-0004", true));
        assert_eq!(num(vec![value_string("-4.8", true), value_string("1", true)]).unwrap(),
            value_string("-4", true));
        assert_eq!(num(vec![value_string("-4.8", true), value_string("-1", true)]).unwrap(),
            value_string("-4", true));
    }
}
