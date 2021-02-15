use pest::{iterators::Pair, Parser};
use chrono::{DateTime, Local, TimeZone};
use crate::c::{SECOND, MINUTE_F64, HOUR_F64, DAY};
use crate::at::At;
use crate::parser::{InputParser, PestRule as Rule};
use std::result::Result;

pub struct Describe;

impl Describe {
    pub fn with(input: &str, utc_timestamp_secs: u64) -> Result<u64, ()> {
        let local_dt = Local.timestamp(utc_timestamp_secs as i64, 0);
        let utc_offset_secs = local_dt.offset().local_minus_utc();

        let total: u64 = match InputParser::parse(Rule::Input, input) {
            Ok(parsed) => {
                let mut total: i64 = 0;

                for expr in parsed {
                    match expr.as_rule() {
                        Rule::AtTime => {
                            total += Describe::at_time_expr(expr, &local_dt);
                        }
                        Rule::DurationExpr => {
                            total += Describe::duration_expr(expr);
                        }
                        _ => {}
                    }
                }

                if total.is_negative() {
                    0
                } else {
                    total as _
                }
            }
            Err(_) => return Err(()),
        };

        Ok(total)
    }

    fn duration_expr(expr: Pair<Rule>) -> i64 {
        let mut needle: f64 = 0.0;

        for prop in expr.into_inner() {
            match prop.as_rule() {
                Rule::Duration => {
                    needle = prop.as_str().parse().unwrap();
                }
                Rule::Hours => {
                    return (needle * HOUR_F64) as _;
                }
                Rule::Minutes => {
                    return (needle * MINUTE_F64) as _;
                }
                Rule::Seconds => {
                    return needle as _;
                }
                _ => {}
            }
        }

        unreachable!()
    }

    fn at_time_expr(expr: Pair<Rule>, now: &DateTime<Local>) -> i64 {
        let at = At::parse(expr);
        let dt_at = at.datetime(now);

        let diff = dt_at.timestamp_millis() - now.timestamp_millis();

        if diff >= 0 {
            diff
        } else {
            DAY + diff
        }
    }
}
