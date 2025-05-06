use crate::functions::num::to_int;

use crate::types::Error;
use crate::types::Error::*;

/* $mul(a,b, ...)
 * Multiply a and b and c and ...
 */
pub fn mul(args: Vec<String>) -> Result<String, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("mul"), args.len()));
    }
    args.iter()
        .try_fold(1i64, |cur, x| {
            cur.checked_mul(to_int(x)).ok_or(Error::OutOfRange)
        })
        .map(|val| val.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            mul(vec![String::from("a")]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("mul"), 1)
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            String::from("9"),
            mul(vec![String::from("3"), String::from("3")])
                .ok()
                .unwrap()
        );
        assert_eq!(
            String::from("27"),
            mul(vec![
                String::from("3"),
                String::from("3"),
                String::from("3")
            ])
            .ok()
            .unwrap()
        );
    }
}
