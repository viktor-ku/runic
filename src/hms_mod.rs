#[cfg(target_arch = "wasm32")]
use crate::c::{HOUR_F64, MINUTE_F64};
#[cfg(target_arch = "wasm32")]
use crate::OpenRunic;
#[cfg(target_arch = "wasm32")]
use js_sys::Array;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const SECOND: u64 = 1;
const MINUTE: u64 = SECOND * 60;
const HOUR: u64 = MINUTE * 60;

#[cfg(not(target_arch = "wasm32"))]
pub fn hms(seconds: u64) -> (u64, u64, u64) {
    let mut secs = seconds.clone();

    let hours = secs / HOUR;
    secs -= hours * HOUR;

    let minutes = secs / MINUTE;
    secs -= minutes * MINUTE;

    (hours, minutes, secs)
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn hms(seconds: f64) -> Array {
    let mut secs = seconds.clone();

    let hours = (secs / HOUR_F64).trunc();
    secs -= hours * HOUR_F64;

    let minutes = (secs / MINUTE_F64).trunc();
    secs -= minutes * MINUTE_F64;

    let arr = Array::new_with_length(3);

    arr.set(0, hours.into());
    arr.set(1, minutes.into());
    arr.set(2, secs.into());

    arr
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod hms {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn _1s() {
        assert_eq!(hms(1), (0, 0, 1));
    }

    #[test]
    fn _1m() {
        assert_eq!(hms(60), (0, 1, 0));
    }

    #[test]
    fn _1h() {
        assert_eq!(hms(3600), (1, 0, 0));
    }

    #[test]
    fn _1h_1m_1s() {
        assert_eq!(hms(3661), (1, 1, 1));
    }

    #[test]
    fn _2h_37m_11s() {
        assert_eq!(hms((2 * 3600) + (37 * 60) + 11), (2, 37, 11));
    }

    #[test]
    fn _5h_11m() {
        assert_eq!(hms((5 * 3600) + (11 * 60)), (5, 11, 0));
    }
}
