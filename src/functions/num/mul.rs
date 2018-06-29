use functions::num::to_int;

use types::Error;
use types::Error::*;

/* $mul(a,b, ...)
 * Multiply a and b and c and ...
 */
pub fn mul(args : Vec<String>) -> Result<String, Error> {
    if args.len() < 2 {
        return Err(InvalidNativeFunctionArgs(String::from("mul"), args.len()));
    }
    Ok(args.iter().fold(1, |cur, x| { cur * to_int(x) }).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
