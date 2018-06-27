use types::Error;
use types::Error::*;
use std;

/* The functions in this section can be used to perform arithmetic on
 * integer numbers. A string will be automatically converted to a number
 * and vice versa. The conversion to a number uses the longest prefix of
 * the string that can be interpreted as number. Leading whitespace is
 * ignored. Decimal points are not supported. Examples:
 *
 * * c3po → 0
 * * 4.8 → 4
 * * -12 → -12
 * * - 12 → 0
 */
fn to_int(s: &str) -> i64 {
    let mut s = String::from(s);
    s = s.trim_left().to_string();
    let negative = if s.starts_with("-") { s = s.split_off(1); -1 } else { 1 };
    let mut num_str = String::from("");
    for i in s.chars() {
        match i {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => num_str.push(i),
            _ => break,
        }
    };
    let mut num = 0;
    if num_str.len() != 0 {
        num = num_str.parse::<i64>().unwrap();
    }
    num * negative
}

/* $add(a,b, ...)
 * Adds a and b and c and ...
 */
pub fn add(args : Vec<String>) -> Result<String, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("add"), args.len()));
    }
    Ok(args.iter().fold(0, |cur, x| { cur + to_int(x) }).to_string())
}

/* $sub(a,b, ...)
 * Subtract a-b-c-...
 */
pub fn sub(args : Vec<String>) -> Result<String, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("sub"), args.len()));
    }
    let mut iter = args.iter();
    let accum = {
        match iter.next() {
            Some(v) => to_int(v),
            None => unreachable!(),
        }
    };
    Ok(iter.fold(accum, |cur, x| { cur - to_int(x) }).to_string())
}

/* $mul(a,b, ...)
 * Multiply a and b and c and ...
 */
pub fn mul(args : Vec<String>) -> Result<String, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("mul"), args.len()));
    }
    Ok(args.iter().fold(1, |cur, x| { cur * to_int(x) }).to_string())
}

/* $div(a,b, ...)
 * Divides a and b and c and ...
 */
pub fn div(args : Vec<String>) -> Result<String, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("div"), args.len()));
    }
    let mut iter = args.iter();
    let accum = {
        match iter.next() {
            Some(v) => to_int(v),
            None => unreachable!(),
        }
    };
    Ok(iter.fold(accum, |cur, x| {
        let i = to_int(x);
        if i != 0 { cur / i } else { i }
    }).to_string())
}

/* $max(a,b, ...)
 * find the maximum of a, b, c, ...
 */
pub fn max(args : Vec<String>) -> Result<String, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("max"), args.len()));
    }
    Ok(args.iter().fold(std::i64::MIN, |cur, x| {
        let i = to_int(x);
        if i > cur { i } else { cur }
    }).to_string())
}

/* $min(a,b, ...)
 * find the maximum of a, b, c, ...
 */
pub fn min(args : Vec<String>) -> Result<String, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("min"), args.len()));
    }
    Ok(args.iter().fold(std::i64::MAX, |cur, x| {
        let i = to_int(x);
        if i < cur { i } else { cur }
    }).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
/* c3po → 0
 * 4.8 → 4
 * -12 → -12
 * - 12 → 0
 */
        /* no digits */
        assert_eq!(0, to_int("c3po"));
        /* no floats */
        assert_eq!(4, to_int("4.8"));
        /* valid number */
        assert_eq!(-12, to_int("-12"));
        /* no whitespace between '-' and number */
        assert_eq!(0, to_int("- 12"));
        /* leading whitespace is ignored */
        assert_eq!(4, to_int(" 4.8"));
    }

    #[test]
    fn test_add() {
        assert_eq!(
            String::from("4"),
            add(vec![
                String::from("2"),
                String::from("2")]
            ).ok().unwrap()
        );
        assert_eq!(
            String::from("6"),
            add(vec![
                String::from("2"),
                String::from("2"),
                String::from("2")]
            ).ok().unwrap()
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            String::from("0"),
            sub(vec![
                String::from("2"),
                String::from("2")]
            ).ok().unwrap()
        );
        assert_eq!(
            String::from("4"),
            sub(vec![
                String::from("8"),
                String::from("2"),
                String::from("2")]
            ).ok().unwrap()
        );
    }

    #[test]
    fn test_div() {
        assert_eq!(
            String::from("16"),
            div(vec![
                String::from("32"),
                String::from("2")]
            ).ok().unwrap()
        );
        assert_eq!(
            String::from("8"),
            div(vec![
                String::from("32"),
                String::from("2"),
                String::from("2")]
            ).ok().unwrap()
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            String::from("9"),
            mul(vec![
                String::from("3"),
                String::from("3")]
            ).ok().unwrap()
        );
        assert_eq!(
            String::from("27"),
            mul(vec![
                String::from("3"),
                String::from("3"),
                String::from("3")]
            ).ok().unwrap()
        );
    }

    #[test]
    fn test_max() {
        assert_eq!(
            String::from("2"),
            max(vec![
                String::from("1"),
                String::from("2")]
            ).ok().unwrap()
        );
        assert_eq!(
            String::from("2"),
            max(vec![
                String::from("-1"),
                String::from("2"),
                String::from("-4")]
            ).ok().unwrap()
        );
    }

    #[test]
    fn test_min() {
        assert_eq!(
            String::from("1"),
            min(vec![
                String::from("1"),
                String::from("2")]
            ).ok().unwrap()
        );
        assert_eq!(
            String::from("-4"),
            min(vec![
                String::from("-1"),
                String::from("2"),
                String::from("-4")]
            ).ok().unwrap()
        );
    }
}
