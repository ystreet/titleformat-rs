
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
fn to_int(s: String) -> i64 {
    let mut s = s.clone();
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
 * Adds a and b.
 */
pub fn add(args : Vec<String>) -> String {
    let mut accum = 0;
    for v in args {
        accum += to_int (v);
    }
    accum.to_string()
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
        assert_eq!(0, to_int(String::from("c3po")));
        /* no floats */
        assert_eq!(4, to_int(String::from("4.8")));
        /* valid number */
        assert_eq!(-12, to_int(String::from("-12")));
        /* no whitespace between '-' and number */
        assert_eq!(0, to_int(String::from("- 12")));
        /* leading whitespace is ignored */
        assert_eq!(4, to_int(String::from(" 4.8")));
    }

    #[test]
    fn test_add() {
        assert_eq!(
            String::from("4"),
            add(vec![
                String::from("2"),
                String::from("2")]
            )
        );
    }
}
