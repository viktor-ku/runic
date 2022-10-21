use crate::describe::Describe;
use chrono::{Local, TimeZone, Utc};
use anyhow::Result;

/// When constructing `Runic`, one can use only specific fields and fill
/// all the others with `None` (in which case they will be computed during `describe`
/// function execution). Note, you can also use builder-like functions or `Default` is also
/// implemented to help you out. This also makes it easier to reuse the same _Runic_ multiple
/// times slightly adjusting on of the fields.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
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

impl<'runic> Runic<'runic> {
    pub fn new(script: &'runic str) -> Self {
        let timestamp = Self::compute_timestamp();

        Self {
            script,
            timestamp: Some(timestamp),
            offset: Some(Self::compute_offset(timestamp)),
        }
    }

    pub fn timestamp(&mut self, timestamp: i64) -> &mut Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn describe(&self) -> Result<u64> {
        let timestamp = self.timestamp.unwrap_or_else(|| Self::compute_timestamp());
        let offset = self
            .offset
            .unwrap_or_else(|| Self::compute_offset(timestamp));

        Describe::with(self.script, timestamp, offset)
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
