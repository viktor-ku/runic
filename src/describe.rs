use crate::at::At;
use crate::c::{DAY, HOUR_F64, MINUTE_F64, SECOND};
use crate::parser::{InputParser, PestRule as Rule};
use chrono::{DateTime, Local, TimeZone};
use pest::{iterators::Pair, Parser};
use std::result::Result;

pub struct Describe {
    timestamp: i64,
    offset: i64,
    local: i64,
}

impl Describe {
    pub fn with(input: &str, timestamp: i64, offset: i64) -> Result<u64, ()> {
        let cx = Self {
            timestamp,
            offset,
            local: timestamp + offset,
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

        Ok(0)
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
