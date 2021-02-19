use crate::describe::Describe;
use crate::OpenRunic;
use chrono::{Local, TimeZone, Utc};
use wasm_bindgen::prelude::*;

/// When constructing `Runic`, one can use only specific fields and fill
/// all the others with `None` (in which case they will be computed during `describe`
/// function execution). Note, you can also use builder-like functions or `Default` is also
/// implemented to help you out. This also makes it easier to reuse the same _Runic_ multiple
/// times slightly adjusting on of the fields.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
#[cfg(not(target_arch = "wasm32"))]
pub struct Runic<'runic> {
    /// The string you want to describe.
    pub script: &'runic str,

    /// Set the timestamp based on which the total duration should
    /// be computed. It expects the number of seconds since Unix epoch minus
    /// leap seconds. Note, that it also accepts negative numbers.
    ///
    /// Takes _now_ at the moment of the `describe` execution If set to `None`.
    pub timestamp: Option<i64>,

    /// Set the utc offset manually. It expects the number of seconds.
    ///
    /// It is not recommended however, because internally
    /// offset will be calculated based on the local time if set to `None` during
    /// the `describe` execution.
    ///
    /// Of course, it might be actually useful in cases where offset is known
    /// in advance.
    pub offset: Option<i32>,
}

#[cfg(not(target_arch = "wasm32"))]
impl<'runic> Runic<'runic> {
    pub fn new(script: &'runic str) -> Self {
        let timestamp = Self::compute_timestamp();

        Self {
            script,
            timestamp: Some(timestamp),
            offset: Some(Self::compute_offset(timestamp)),
        }
    }

    #[inline]
    pub fn timestamp(&mut self, timestamp: i64) -> &mut Self {
        self.timestamp = Some(timestamp);
        self
    }

    #[inline]
    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn describe(&self) -> OpenRunic {
        let timestamp = self.timestamp.unwrap_or_else(|| Self::compute_timestamp());
        let offset = self
            .offset
            .unwrap_or_else(|| Self::compute_offset(timestamp));

        OpenRunic::new(Describe::with(self.script, timestamp, offset).unwrap_or(0))
    }

    #[inline]
    fn compute_timestamp() -> i64 {
        Utc::now().timestamp()
    }

    #[inline]
    fn compute_offset(base: i64) -> i32 {
        Local.timestamp(base, 0).offset().local_minus_utc()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for Runic<'_> {
    fn default() -> Self {
        Self {
            script: "",
            timestamp: None,
            offset: None,
        }
    }
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
pub struct Runic {
    script: String,
    timestamp: i64,
    offset: i32,
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
impl Runic {
    pub fn new(script: &str) -> Self {
        let timestamp = Self::compute_timestamp();

        Self {
            script: String::from(script),
            timestamp: timestamp,
            offset: Self::compute_offset(timestamp),
        }
    }

    #[inline]
    pub fn timestamp(&mut self, timestamp: f64) {
        self.timestamp = timestamp as _;
    }

    #[inline]
    pub fn offset(&mut self, offset: i32) {
        self.offset = offset;
    }

    pub fn describe(&self) -> OpenRunic {
        OpenRunic::new(Describe::with(&self.script, self.timestamp, self.offset).unwrap_or(0))
    }

    pub fn clone(&self) -> Self {
        Self {
            script: self.script.clone(),
            timestamp: self.timestamp.clone(),
            offset: self.offset.clone(),
        }
    }

    #[inline]
    fn compute_timestamp() -> i64 {
        Utc::now().timestamp()
    }

    #[inline]
    fn compute_offset(base: i64) -> i32 {
        Local.timestamp(base, 0).offset().local_minus_utc()
    }
}
