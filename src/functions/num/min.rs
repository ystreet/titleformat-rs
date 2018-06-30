use functions::num::to_int;

use types::Error;
use types::Error::*;
use std;

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
    fn wrong_n_arguments() {
        assert_eq!(min(
            vec![
                String::from("a")
            ]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("min"), 1));
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
