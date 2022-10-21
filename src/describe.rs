use crate::at::At;
use crate::c::{DAY, HOUR_F64, MINUTE_F64};
use crate::parser::{PestRule as Rule, ScriptParser};
use crate::script_timezone::ScriptTimezone;
use chrono::{FixedOffset, TimeZone, Timelike};
use pest::{iterators::Pair, Parser};
use std::result::Result;

pub struct Describe {
    utc: i64,
    offset: i32,
}

impl Describe {
    pub fn with(script: &str, timestamp: i64, offset: i32) -> Result<u64, ()> {
        let cx = Self {
            utc: timestamp,
            offset,
        };

        let mut at: Option<Pair<Rule>> = None;
        let mut duration_total = 0;

        match ScriptParser::parse(Rule::Input, script) {
            Ok(parsed) => {
                println!("{:#?}", parsed);

                for expr in parsed {
                    match expr.as_rule() {
                        Rule::AtTimeExpr => {
                            at = Some(expr);
                        }
                        Rule::DurationExpr => {
                            duration_total += Self::compute_duration_from_timeout_expr(expr);
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                panic!("{}", e)
            },
        };

        let at_total = match at {
            Some(expr) => cx.compute_duration_from_target_expr(expr),
            None => 0,
        };

        let total = {
            let total = at_total + duration_total;

            if total.is_negative() {
                0
            } else {
                total
            }
        };

        Ok(total as _)
    }

    fn compute_duration_from_target_expr(&self, expr: Pair<Rule>) -> i64 {
        let at = At::parse(expr);

        let target = {
            let offset = match at.script_timezone {
                ScriptTimezone::Mirror => FixedOffset::east(self.offset),
                ScriptTimezone::Custom(script_offset) => FixedOffset::east(script_offset),
            };

            let target = offset.timestamp(self.utc, 0);
            let target = target.with_hour(at.hours).unwrap();
            let target = target.with_minute(at.minutes).unwrap();
            let target = target.with_second(0).unwrap();
            let target = target.with_nanosecond(0).unwrap();

            target.timestamp()
        };

        let duration = target - self.utc;

        if duration.is_negative() {
            DAY + duration
        } else {
            duration
        }
    }

    fn compute_duration_from_timeout_expr(expr: Pair<Rule>) -> i64 {
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
