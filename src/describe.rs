use pest::{iterators::Pair, Parser};
use chrono::{DateTime, Local, TimeZone};
use crate::c::{SECOND, MINUTE, HOUR, DAY};
use crate::At;
use crate::parser::{InputParser, PestRule};
use std::result::Result;

pub struct Describe;

impl Describe {
    pub fn with(input: &str, utc_timestamp_secs: u64) -> Result<u64, ()> {
        let local_dt = Local.timestamp(utc_timestamp_secs as i64, 0);
        let utc_offset_secs = local_dt.offset().local_minus_utc();

        let total: f64 = match InputParser::parse(PestRule::Input, input) {
            Ok(parsed) => {
                let mut total = 0.0;

                for expr in parsed {
                    match expr.as_rule() {
                        PestRule::AtTime => {
                            total += Describe::at_time_expr(expr, &local_dt);
                        }
                        PestRule::DurationExpr => {
                            total += Describe::duration_expr(expr);
                        }
                        _ => {}
                    }
                }

                if total.is_sign_negative() {
                    0.0
                } else {
                    total
                }
            }
            Err(_) => return Err(()),
        };

        Ok(total as _)
    }

    fn duration_expr(expr: Pair<PestRule>) -> f64 {
        let mut needle = 0.0;

        for prop in expr.into_inner() {
            match prop.as_rule() {
                PestRule::Duration => {
                    needle = prop.as_str().parse().unwrap();
                }
                PestRule::Hours => {
                    return needle * HOUR;
                }
                PestRule::Minutes => {
                    return needle * MINUTE;
                }
                PestRule::Seconds => {
                    return needle.trunc() * SECOND;
                }
                _ => {}
            }
        }

        unreachable!()
    }

    fn at_time_expr(expr: Pair<PestRule>, now: &DateTime<Local>) -> f64 {
        let at = At::parse(expr);
        let dt_at = at.datetime(now);
        let diff = dt_at.timestamp_millis() - now.timestamp_millis();
        if diff >= 0 {
            diff as f64
        } else {
            DAY + (diff as f64)
        }
    }
}
