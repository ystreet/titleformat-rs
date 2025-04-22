use crate::functions::num::to_int;

use crate::types::Error;
use crate::types::Error::*;

/* $sub(a,b, ...)
 * Subtract a-b-c-...
 */
pub fn sub(args: Vec<String>) -> Result<String, Error> {
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
    Ok(iter.fold(accum, |cur, x| cur - to_int(x)).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            sub(vec![String::from("a")]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("sub"), 1)
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            String::from("0"),
            sub(vec![String::from("2"), String::from("2")])
                .ok()
                .unwrap()
        );
        assert_eq!(
            String::from("4"),
            sub(vec![
                String::from("8"),
                String::from("2"),
                String::from("2")
            ])
            .ok()
            .unwrap()
        );
    }
}
