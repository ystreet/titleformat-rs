use crate::functions::num::to_int;

use crate::environment::value_string;
use crate::environment::Value;
use crate::functions::control::or::or;
use crate::types::Error;
use crate::types::Error::*;

/* $eq(a,b)
 * Return True if a > b
 */
pub fn eq(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(InvalidNativeFunctionArgs(String::from("eq"), args.len()));
    }
    let a = to_int(&args[0].val);
    let b = to_int(&args[1].val);
    Ok(value_string("", a == b))
}

fn map_err_func_name(from: Error, into: &str) -> Error {
    match from {
        InvalidNativeFunctionArgs(_, len) => InvalidNativeFunctionArgs(into.to_string(), len),
        _ => from,
    }
}

/* $ne(a,b)
 * Return True if a > b
 */
pub fn ne(args: Vec<Value>) -> Result<Value, Error> {
    eq(args)
        .map(|v| value_string(&v.val, !v.cond))
        .map_err(|e| map_err_func_name(e, "ne"))
}

/* $gt(a,b)
 * Return True if a > b
 */
pub fn gt(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(InvalidNativeFunctionArgs(String::from("gt"), args.len()));
    }
    let a = to_int(&args[0].val);
    let b = to_int(&args[1].val);
    Ok(value_string("", a > b))
}

/* $gte(a,b)
 * Return True if a > b
 */
pub fn gte(args: Vec<Value>) -> Result<Value, Error> {
    or(vec![
        eq(args.clone()).map_err(|e| map_err_func_name(e, "gte"))?,
        gt(args).map_err(|e| map_err_func_name(e, "gte"))?,
    ])
    .map_err(|e| map_err_func_name(e, "gte"))
}

/* $lt(a,b)
 * Return True if a > b
 */
pub fn lt(args: Vec<Value>) -> Result<Value, Error> {
    gte(args)
        .map(|v| value_string(&v.val, !v.cond))
        .map_err(|e| map_err_func_name(e, "lt"))
}

/* $lte(a,b)
 * Return True if a > b
 */
pub fn lte(args: Vec<Value>) -> Result<Value, Error> {
    gt(args)
        .map(|v| value_string(&v.val, !v.cond))
        .map_err(|e| map_err_func_name(e, "lte"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            eq(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("eq"), 0)
        );
        assert_eq!(
            ne(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("ne"), 0)
        );
        assert_eq!(
            gt(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("gt"), 0)
        );
        assert_eq!(
            gte(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("gte"), 0)
        );
        assert_eq!(
            lt(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("lt"), 0)
        );
        assert_eq!(
            lte(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("lte"), 0)
        );
    }

    #[test]
    fn test_eq() {
        assert_eq!(
            value_string("", false),
            eq(vec![value_string("1", true), value_string("2", true)])
                .ok()
                .unwrap()
        );
        assert_eq!(
            value_string("", true),
            eq(vec![value_string("1", true), value_string("1", true)])
                .ok()
                .unwrap()
        );
    }

    #[test]
    fn test_ne() {
        assert_eq!(
            value_string("", true),
            ne(vec![value_string("1", true), value_string("2", true)])
                .ok()
                .unwrap()
        );
        assert_eq!(
            value_string("", false),
            ne(vec![value_string("1", true), value_string("1", true)])
                .ok()
                .unwrap()
        );
    }

    #[test]
    fn test_gt() {
        assert_eq!(
            value_string("", false),
            gt(vec![value_string("1", true), value_string("2", true)])
                .ok()
                .unwrap()
        );
        assert_eq!(
            value_string("", false),
            gt(vec![value_string("1", true), value_string("1", true)])
                .ok()
                .unwrap()
        );
        assert_eq!(
            value_string("", true),
            gt(vec![value_string("2", true), value_string("1", true)])
                .ok()
                .unwrap()
        );
    }

    #[test]
    fn test_gte() {
        assert_eq!(
            value_string("", false),
            gte(vec![value_string("1", true), value_string("2", true)])
                .ok()
                .unwrap()
        );
        assert_eq!(
            value_string("", true),
            gte(vec![value_string("1", true), value_string("1", true)])
                .ok()
                .unwrap()
        );
        assert_eq!(
            value_string("", true),
            gte(vec![value_string("2", true), value_string("1", true)])
                .ok()
                .unwrap()
        );
    }

    #[test]
    fn test_lt() {
        assert_eq!(
            value_string("", true),
            lt(vec![value_string("1", true), value_string("2", true)])
                .ok()
                .unwrap()
        );
        assert_eq!(
            value_string("", false),
            lt(vec![value_string("1", true), value_string("1", true)])
                .ok()
                .unwrap()
        );
        assert_eq!(
            value_string("", false),
            lt(vec![value_string("2", true), value_string("1", true)])
                .ok()
                .unwrap()
        );
    }

    #[test]
    fn test_lte() {
        assert_eq!(
            value_string("", true),
            lte(vec![value_string("1", true), value_string("2", true)])
                .ok()
                .unwrap()
        );
        assert_eq!(
            value_string("", true),
            lte(vec![value_string("1", true), value_string("1", true)])
                .ok()
                .unwrap()
        );
        assert_eq!(
            value_string("", false),
            lte(vec![value_string("2", true), value_string("1", true)])
                .ok()
                .unwrap()
        );
    }
}
