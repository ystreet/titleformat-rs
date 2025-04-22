use crate::environment::value_string;
use crate::environment::Value;
use crate::types::Error;
use crate::types::Error::*;

use unicode_segmentation::UnicodeSegmentation;

pub fn upper(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(InvalidNativeFunctionArgs(String::from("upper"), args.len()));
    }

    Ok(value_string(
        args[0].val.to_uppercase().as_str(),
        args[0].cond,
    ))
}

pub fn lower(args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(InvalidNativeFunctionArgs(String::from("lower"), args.len()));
    }

    Ok(value_string(
        args[0].val.to_lowercase().as_str(),
        args[0].cond,
    ))
}

/*
 * $firstalphachar(text,nonalpha="#")
 *
 * Returns the first character of text. If text is not an alphabetic character nonalpha is returned instead.
 */
pub fn firstalphachar(args: Vec<Value>) -> Result<Value, Error> {
    let missing = match args.len() {
        1 => String::from("#"),
        2 => args[1].val.clone(),
        _ => return Err(InvalidNativeFunctionArgs(String::from("lower"), args.len())),
    };

    let s = args[0].val.as_str();
    let c = UnicodeSegmentation::graphemes(s, true)
        .next()
        .map_or(missing.clone(), |seg| {
            seg.chars().next().map_or(missing.clone(), |c| {
                if c.is_alphabetic() {
                    c.to_string()
                } else {
                    missing
                }
            })
        });

    Ok(value_string(&c, args[0].cond))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            upper(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("upper"), 0)
        );
        assert_eq!(
            lower(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("lower"), 0)
        );
    }

    #[test]
    fn test_upper() {
        assert_eq!(
            upper(vec![value_string("AbcD", true)]).unwrap(),
            value_string("ABCD", true)
        );
    }

    #[test]
    fn test_lower() {
        assert_eq!(
            lower(vec![value_string("AbcD", true)]).unwrap(),
            value_string("abcd", true)
        );
    }

    #[test]
    fn test_firstalphachar() {
        assert_eq!(
            firstalphachar(vec![value_string("AbcD", true)]).unwrap(),
            value_string("A", true)
        );
        assert_eq!(
            firstalphachar(vec![value_string("1", true)]).unwrap(),
            value_string("#", true)
        );
        assert_eq!(
            firstalphachar(vec![value_string("1", true), value_string("@", true)]).unwrap(),
            value_string("@", true)
        );
    }
}
