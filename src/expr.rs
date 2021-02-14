use pest::{iterators::Pair};
use chrono::{DateTime, Local};
use crate::c::{SECOND, MINUTE, HOUR, DAY};
use crate::At;
use crate::PestRule;

pub(crate) fn duration_expr(expr: Pair<PestRule>) -> f64 {
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

pub(crate) fn at_time_expr(expr: Pair<PestRule>, now: &DateTime<Local>) -> f64 {
    let at = At::parse(expr);
    let dt_at = at.datetime(now);
    let diff = dt_at.timestamp_millis() - now.timestamp_millis();
    if diff >= 0 {
        diff as f64
    } else {
        DAY + (diff as f64)
    }
}
