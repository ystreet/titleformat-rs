use environment::{Environment, Value, value_string};
use types::Error;
use types::Error::*;
use functions::num::to_int;

/*
 * $meta(name)
 * Returns value of tag called name. If multiple values of that tag exist,
 * they are concatenated with ", " as separator.
 * Example: $meta(artist) → "He, She, It"
 *
 * $meta(name,n)
 * Returns value of n-th (0,1,2 and so on) tag called name.
 * Example: $meta(artist,1) → "She" 
 */
pub fn meta (env : &Environment, args : Vec<Value>) -> Result<Value, Error> {
  match args.len() {
    1 => Ok(env.meta(&args[0].val)),
    2 => Ok(env.meta_i(&args[0].val, to_int(&args[1].val) as usize)),
    _ => Err(InvalidNativeFunctionArgs(String::from("meta"), args.len())),
  }
}

/*
 * $meta_sep(name,sep)
 * Returns value of tag called name. If multiple values of that tag exist, they are concatenated with sep as separator.
 * Example: $meta_sep(artist,' + ') → "He + She + It"
 *
 * $meta_sep(name,sep,lastsep)
 * Returns value of tag called name. If multiple values of that tag exist,
 * they are concatenated with sep as separator between all but the last two
 * values which are concatenated with lastsep.
 * Example: $meta_sep(artist,', ',', and ') → "He, She, and It"
 */
pub fn meta_sep (env : &Environment, args : Vec<Value>) -> Result<Value, Error> {
  match args.len() {
    2 => Ok(env.meta_sep(&args[0].val, &args[1].val)),
    3 => Ok(env.meta_sep_with_last(&args[0].val, &args[1].val, &args[2].val)),
    _ => Err(InvalidNativeFunctionArgs(String::from("meta_sep"), args.len())),
  }
}

/*
 * $meta_test(...)
 * Returns 1, if all given tags exist, undefined otherwise.
 * Example: $meta_test(artist,title) → true
 */
pub fn meta_test (env : &Environment, args : Vec<Value>) -> Result<Value, Error> {
  match args.len() {
    0 => Err(InvalidNativeFunctionArgs(String::from("meta_num"), args.len())),
    _ => Ok(value_string("", args.iter().fold(true, |c, i| {
      c && env.meta_num(&i.val) > 0
    }))),
  }
}

/*
 * $meta_num(name)
 * Returns the number of values for the tag called name.
 * Example: $meta_num(artist) → 3
 */
pub fn meta_num (env : &Environment, args : Vec<Value>) -> Result<Value, Error> {
  match args.len() {
    1 => Ok(value_string(&env.meta_num(&args[0].val).to_string(), true)),
    _ => Err(InvalidNativeFunctionArgs(String::from("meta_num"), args.len())),
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(meta(&Environment::new(HashMap::new()), vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("meta"), 0));
        assert_eq!(meta_sep(&Environment::new(HashMap::new()), vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("meta_sep"), 0));
        assert_eq!(meta_num(&Environment::new(HashMap::new()), vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("meta_num"), 0));
    }

    #[test]
    fn test_meta() {
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![
            String::from("0"),
            String::from("1"),
            String::from("2"),
            String::from("3")
        ]);
        let env = Environment::new(m);
        assert_eq!(meta(&env.clone(),
                vec![
                    value_string("a", true)
                ]
            ).unwrap(),
            value_string("0, 1, 2, 3", true)
        );
        assert_eq!(meta(&env.clone(),
                vec![
                    value_string("a", true),
                    value_string("1", true)
                ]
            ).unwrap(),
            value_string("1", true)
        );
    }

    #[test]
    fn test_meta_sep() {
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![
            String::from("0"),
            String::from("1"),
            String::from("2"),
            String::from("3")
        ]);
        let env = Environment::new(m);
        assert_eq!(meta_sep(&env.clone(),
                vec![
                    value_string("a", true),
                    value_string("|", true)
                ]
            ).unwrap(),
            value_string("0|1|2|3", true)
        );
        assert_eq!(meta_sep(&env.clone(),
                vec![
                    value_string("a", true),
                    value_string("|", true),
                    value_string("^", true)
                ]
            ).unwrap(),
            value_string("0|1|2^3", true)
        );
    }

    #[test]
    fn test_meta_num() {
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![
            String::from("0"),
            String::from("1"),
            String::from("2"),
            String::from("3")
        ]);
        let env = Environment::new(m);
        assert_eq!(meta_num(&env.clone(),
                vec![
                    value_string("a", true)
                ]
            ).unwrap(),
            value_string("4", true)
        );
    }

    #[test]
    fn test_meta_test() {
        let mut m = HashMap::new();
        m.insert(String::from("a"), vec![
            String::from("0"),
            String::from("1"),
            String::from("2"),
            String::from("3")
        ]);
        m.insert(String::from("b"), vec![
            String::from("4"),
        ]);
        let env = Environment::new(m);
        assert_eq!(meta_test(&env.clone(),
                vec![
                    value_string("a", true),
                    value_string("b", true)
                ]
            ).unwrap(),
            value_string("", true)
        );
    }
}
