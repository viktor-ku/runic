use crate::c::{SECOND, MINUTE, HOUR};
use crate::describe::Describe;

#[derive(Debug, PartialEq)]
pub struct OpenRunic {
    total_secs: u64,
}

impl OpenRunic {
    #[inline]
    pub fn total(&self) -> f64 {
        self.total_secs as f64
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

    pub fn describe(input: &str, now_utc_secs: u64) -> Self {
        let total = Describe::with(input, now_utc_secs).unwrap_or(0);

        Self { total_secs: total }
    }
}
