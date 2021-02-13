#[macro_use]
extern crate pest_derive;

use pest::{iterators::Pair, Parser};
use wasm_bindgen::prelude::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct InputParser;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Runic {
    total: f64,
}

const SECOND: f64 = 1_000.0;
const MINUTE: f64 = 60.0 * SECOND;
const HOUR: f64 = 60.0 * MINUTE;
const DAY: f64 = 24.0 * HOUR;

mod at;
use at::At;

use chrono::{DateTime, Local, TimeZone};

#[wasm_bindgen]
impl Runic {
    #[inline]
    pub fn total(&self) -> f64 {
        self.total
    }

    pub fn translate(ms: f64) -> String {
        let mut ms = ms;

        let hours = (ms / HOUR).trunc();
        ms -= hours * HOUR;

        let minutes = (ms / MINUTE).trunc();
        ms -= minutes * MINUTE;

        let seconds = (ms / SECOND).trunc();

        format!("{}h {}m {}s", hours, minutes, seconds)
    }

    fn duration_expr(expr: Pair<Rule>) -> f64 {
        let mut needle = 0.0;

        for prop in expr.into_inner() {
            match prop.as_rule() {
                Rule::Duration => {
                    needle = prop.as_str().parse().unwrap();
                }
                Rule::Hours => {
                    return needle * HOUR;
                }
                Rule::Minutes => {
                    return needle * MINUTE;
                }
                Rule::Seconds => {
                    return needle.trunc() * SECOND;
                }
                _ => {}
            }
        }

        unreachable!()
    }

    fn at_time_expr(expr: Pair<Rule>, now: &DateTime<Local>) -> f64 {
        let at = At::parse(expr);
        let dt_at = at.datetime(now);
        let diff = dt_at.timestamp_millis() - now.timestamp_millis();
        if diff >= 0 {
            diff as f64
        } else {
            DAY + (diff as f64)
        }
    }

    #[inline]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn describe(input: &str, now_utc_secs: u64) -> Self {
        Self::_describe(input, &Local.timestamp(now_utc_secs as i64, 0))
    }

    #[cfg(target_arch = "wasm32")]
    pub fn describe(input: &str, now_utc_ms: f64) -> Self {
        Self::_describe(input, &Local.timestamp_millis(now_utc_ms as i64))
    }

    fn _describe(input: &str, now: &DateTime<Local>) -> Self {
        let total = match InputParser::parse(Rule::Input, input) {
            Ok(parsed) => {
                let mut total = 0.0;

                for expr in parsed {
                    match expr.as_rule() {
                        Rule::AtTime => {
                            total += Self::at_time_expr(expr, now);
                        }
                        Rule::DurationExpr => {
                            total += Self::duration_expr(expr);
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
            Err(_) => panic!("something went wrong"),
        };

        Self { total }
    }
}
