use crate::at::At;
use crate::c::{DAY, HOUR_F64, MINUTE_F64};
use crate::parser::{InputParser, PestRule as Rule};
use chrono::{FixedOffset, TimeZone, Timelike};
use pest::{iterators::Pair, Parser};
use std::result::Result;

pub struct Describe {
    utc: i64,
    offset: i32,
    local: i64,
}

impl Describe {
    pub fn with(input: &str, timestamp: i64, offset: i32) -> Result<u64, ()> {
        let cx = Self {
            utc: timestamp,
            offset,
            local: timestamp + (offset as i64),
        };

        let mut at: Option<At> = None;
        let mut duration_total = 0;

        match InputParser::parse(Rule::Input, input) {
            Ok(parsed) => {
                for expr in parsed {
                    match expr.as_rule() {
                        Rule::AtTime => {
                            at = Some(At::parse(expr));
                        }
                        Rule::DurationExpr => {
                            duration_total += Self::duration_expr(expr);
                        }
                        _ => {}
                    }
                }
            }
            Err(_) => return Err(()),
        };

        let at_total = match at {
            Some(at) => cx.duration_until(&at),
            None => 0,
        };

        let total = {
            let total = at_total + duration_total;

            if total.is_negative() {
                0
            } else {
                total
            }
        } as _;

        Ok(total)
    }

    fn duration_until(&self, at: &At) -> i64 {
        let at = {
            let offset = FixedOffset::east(self.offset);
            let dt = offset.timestamp(self.utc, 0);
            let dt = dt.with_hour(at.0).unwrap();
            let dt = dt.with_minute(at.1).unwrap();
            let dt = dt.with_second(0).unwrap();
            let dt = dt.with_nanosecond(0).unwrap();
            dt.timestamp()
        };

        let diff = at - self.local;

        if diff.is_negative() {
            DAY + diff
        } else {
            diff
        }
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
}
