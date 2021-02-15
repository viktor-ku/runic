#[macro_use]
extern crate pest_derive;

use wasm_bindgen::prelude::*;

mod parser;

mod at;
use at::At;

mod c;
use c::{SECOND, MINUTE, HOUR};

mod describe;
use describe::Describe;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Runic {
    total_secs: u64,
}

#[wasm_bindgen]
impl Runic {
    #[inline]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn total(&self) -> f64 {
        self.total_secs as f64
    }

    #[cfg(target_arch = "wasm32")]
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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn describe(input: &str, now_utc_secs: u64) -> Self {
        let total = Describe::with(input, now_utc_secs).unwrap_or(0);

        Self { total_secs: total }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn describe(input: &str, now_utc_ms: f64) -> Self {
        if (now_utc_ms.is_sign_negative()) {
            return Self { total_secs: 0 }
        }

        let now_utc_secs = (now_utc_ms / 1000.0) as u64;

        let total_secs = Describe::with(input, now_utc_secs).unwrap_or(0);

        Self { total_secs }
    }
}
