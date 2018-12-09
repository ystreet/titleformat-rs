use types::Error;
use types::Error::*;
use environment::Value;
use environment::value_string;
use functions::num::to_int;

fn split_first(s : String, args : Vec<String>) -> Option<(String, String)> {
    for pre in args {
        if s.starts_with(&pre) {
            let splat = s.split_at(pre.bytes().len());
            return Some((String::from(splat.0), String::from(splat.1)));
        }
    }
    None
}

pub fn stripprefix(args : Vec<Value>) -> Result<Value, Error> {
    if args.len() == 0 {
        return Err(InvalidNativeFunctionArgs(String::from("stripprefix"), args.len()))
    }
    let mut s = String::from(args[0].val.clone());
    if args.len() == 1 {
        match split_first (s.clone(), vec![String::from("A "), String::from("The ")]) {
            None => {},
            Some((_, new)) => s = new,
        }
    }
    match split_first (s.clone(), (&args[1..]).iter().map(|ref val| {
                val.val.clone()
            }).collect()) {
        None => {},
        Some((_, new)) => s = new,
    }
    Ok(value_string (s.as_str(), args[0].cond))
}

pub fn swapprefix(args : Vec<Value>) -> Result<Value, Error> {
    if args.len() == 0 {
        return Err(InvalidNativeFunctionArgs(String::from("swapprefix"), args.len()))
    }
    let mut s = String::from(args[0].val.clone());
    if args.len() == 1 {
        match split_first (s.clone(), vec![String::from("A "), String::from("The ")]) {
            None => {},
            Some((s1, s2)) => s = s2 + ", " + &s1.trim_right(),
        }
    }
    match split_first (s.clone(), (&args[1..]).iter().map(|ref val| {
                val.val.clone()
            }).collect()) {
        None => {},
        Some((s1, s2)) => s = s2 + ", " + &s1.trim_right(),
    }
    Ok(value_string (s.as_str(), args[0].cond))
}

pub fn cut(args : Vec<Value>) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(InvalidNativeFunctionArgs(String::from("cut"), args.len()))
    }
    let str_len = to_int (&args[1].val);
    let len = if str_len < 0 {
         0
    } else {
        str_len as usize
    };
    let res: String = args[0].val.chars().take(len).collect();
    Ok(value_string (&res, args[0].cond))
}

pub fn left(args : Vec<Value>) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(InvalidNativeFunctionArgs(String::from("left"), args.len()))
    }
    cut (args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(stripprefix(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("stripprefix"), 0));
        assert_eq!(swapprefix(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("swapprefix"), 0));
        assert_eq!(cut(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("cut"), 0));
        assert_eq!(left(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("left"), 0));
    }

    #[test]
    fn test_stripprefix() {
        assert_eq!(stripprefix(vec![value_string ("A thing", true)]).unwrap(),
            value_string("thing", true));

        /* don't doubly strip */
        assert_eq!(stripprefix(vec![
                value_string ("A thing", true),
                value_string ("A t", true),
                value_string ("hi", true),
            ]).unwrap(),
            value_string("hing", true));
    }

    #[test]
    fn test_swapprefix() {
        assert_eq!(swapprefix(vec![value_string ("A thing", true)]).unwrap(),
            value_string("thing, A", true));

        /* don't doubly swap */
        assert_eq!(swapprefix(vec![
                value_string ("A thing", true),
                value_string ("A t", true),
                value_string ("hi", true),
            ]).unwrap(),
            value_string("hing, A t", true));
    }

    #[test]
    fn test_cut() {
        assert_eq!(cut(vec![
                value_string ("A thing", true),
                value_string ("2", true)]).unwrap(),
            value_string("A ", true));
        assert_eq!(cut(vec![
                value_string ("A thing", true),
                value_string ("non", true)]).unwrap(),
            value_string("", true));
    }
}
