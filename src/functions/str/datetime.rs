use crate::environment::value_string;
use crate::environment::Value;
use crate::types::Error;
use crate::types::Error::*;

use iso_8601::{ApproxDate, DateTime, PartialDateTime};
use std::str::FromStr;

pub fn year(args: Vec<Value>) -> Result<Value, Error> {
    match args.len() {
        1 => {
            let val = &args[0];
            let date = match PartialDateTime::from_str(&val.val) {
                Ok(date) => date,
                Err(_) => return Ok(value_string("", val.cond)),
            };

            let year = match date {
                PartialDateTime::Date(ApproxDate::YMD(ymd)) => ymd.year,
                PartialDateTime::Date(ApproxDate::YM(ym)) => ym.year,
                PartialDateTime::Date(ApproxDate::Y(y)) => y.year,
                PartialDateTime::Date(ApproxDate::WD(wd)) => wd.year,
                PartialDateTime::Date(ApproxDate::W(w)) => w.year,
                PartialDateTime::Date(ApproxDate::O(o)) => o.year,
                PartialDateTime::DateTime(DateTime {
                    date: ApproxDate::YMD(ymd),
                    time: _,
                }) => ymd.year,
                PartialDateTime::DateTime(DateTime {
                    date: ApproxDate::YM(ym),
                    time: _,
                }) => ym.year,
                PartialDateTime::DateTime(DateTime {
                    date: ApproxDate::Y(y),
                    time: _,
                }) => y.year,
                PartialDateTime::DateTime(DateTime {
                    date: ApproxDate::WD(wd),
                    time: _,
                }) => wd.year,
                PartialDateTime::DateTime(DateTime {
                    date: ApproxDate::W(w),
                    time: _,
                }) => w.year,
                PartialDateTime::DateTime(DateTime {
                    date: ApproxDate::O(o),
                    time: _,
                }) => o.year,
                _ => return Ok(value_string("", val.cond)),
            };
            Ok(value_string(year.to_string().as_str(), val.cond))
        }
        _ => Err(InvalidNativeFunctionArgs(String::from("year"), args.len())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_n_arguments() {
        assert_eq!(
            year(vec![]).err().unwrap(),
            InvalidNativeFunctionArgs(String::from("year"), 0)
        );
    }

    #[test]
    fn test_year() {
        assert_eq!(
            year(vec![value_string("2000-06-04", true)]).unwrap(),
            value_string("2000", true)
        );
        assert_eq!(
            year(vec![value_string("2000-06", true)]).unwrap(),
            value_string("2000", true)
        );
        assert_eq!(
            year(vec![value_string("2000", true)]).unwrap(),
            value_string("2000", true)
        );
        assert_eq!(
            year(vec![value_string("abcd", true)]).unwrap(),
            value_string("", true)
        );
    }
}
