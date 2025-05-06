use crate::functions::num::to_int;

use crate::types::Error;
use crate::types::Error::*;

/* $add(a,b, ...)
 * Adds a and b and c and ...
 */
pub fn add(args: Vec<String>) -> Result<String, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("add"), args.len()));
    }
    args.iter()
        .try_fold(0i64, |cur, x| {
            cur.checked_add(to_int(x)).ok_or(Error::OutOfRange)
        })
        .map(|val| val.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            add(vec![String::from("a")]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("add"), 1)
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            String::from("4"),
            add(vec![String::from("2"), String::from("2")])
                .ok()
                .unwrap()
        );
        assert_eq!(
            String::from("6"),
            add(vec![
                String::from("2"),
                String::from("2"),
                String::from("2")
            ])
            .ok()
            .unwrap()
        );
    }
}
