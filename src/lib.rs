#[macro_use]
extern crate pest_derive;

use pest::{Parser};
use wasm_bindgen::prelude::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct InputParser;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Runic {
    total: f64,
}

mod at;
use at::At;

mod c;
use c::{SECOND, MINUTE, HOUR};

mod expr;
use expr::{at_time_expr, duration_expr};

use chrono::{DateTime, Local, TimeZone};

pub(crate) type PestRule = Rule;

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
                            total += at_time_expr(expr, now);
                        }
                        Rule::DurationExpr => {
                            total += duration_expr(expr);
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
