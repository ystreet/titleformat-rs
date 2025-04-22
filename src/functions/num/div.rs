use crate::functions::num::to_int;

use crate::types::Error;
use crate::types::Error::*;

/* $div(a,b, ...)
 * Divides a and b and c and ...
 */
pub fn div(args: Vec<String>) -> Result<String, Error> {
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
    Ok(iter
        .fold(accum, |cur, x| {
            let i = to_int(x);
            if i != 0 {
                cur / i
            } else {
                i
            }
        })
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            div(vec![String::from("a")]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("div"), 1)
        );
    }

    #[test]
    fn test_div() {
        assert_eq!(
            String::from("16"),
            div(vec![String::from("32"), String::from("2")])
                .ok()
                .unwrap()
        );
        assert_eq!(
            String::from("8"),
            div(vec![
                String::from("32"),
                String::from("2"),
                String::from("2")
            ])
            .ok()
            .unwrap()
        );
    }
}
