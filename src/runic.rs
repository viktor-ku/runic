use crate::describe::Describe;
use crate::OpenRunic;
use chrono::{Local, TimeZone, Utc};

pub struct Runic<'runic> {
    /// The string you want to describe.
    pub script: &'runic str,

    /// Set the timestamp based on which the total duration should
    /// be computed. It expects the number of seconds since Unix epoch minus
    /// leap seconds. Note, that it also accepts negative number.
    ///
    /// Takes _now_ at the moment of `describe` execution If set to `None`.
    pub timestamp: i64,

    /// Set the utc offset manually. It expects the number of seconds.
    ///
    /// It is not recommended however, because internally
    /// offset will be calculated based on the local time if set to `None`.
    ///
    /// Of course, it might be actually useful in cases where offset is known
    /// in advance.
    pub offset: i32,
}

impl<'runic> Runic<'runic> {
    pub fn new(script: &'runic str) -> Self {
        let timestamp = Self::compute_timestamp();

        Self {
            script,
            timestamp,
            offset: Self::compute_offset(timestamp),
        }
    }

    #[inline]
    pub fn timestamp(&mut self, timestamp: i64) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    #[inline]
    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn describe(&self) -> OpenRunic {
        let total = Describe::with(self.script, self.timestamp, self.offset).unwrap_or(0);

        OpenRunic::new(total)
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
