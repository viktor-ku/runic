use crate::c::{SECOND, MINUTE, HOUR};
use crate::describe::Describe;
use crate::OpenRunic;

pub struct Runic<'runic> {
    /// The string you want to describe.
    pub script: &'runic str,

    /// Set the timestamp based on which the total duration should
    /// be computed. It expects the number of seconds since Unix epoch minus
    /// leap seconds.
    ///
    /// Takes _now_ at the moment of `describe` execution If set to `None`.
    pub timestamp: Option<u64>,

    /// Set the utc offset manually. It expects the number of seconds.
    ///
    /// It is not recommended however, because internally
    /// offset will be calculated based on the local time if set to `None`.
    ///
    /// Of course, it might be actually useful in cases where offset is known
    /// in advance.
    pub utc_offset: Option<i32>,
}

impl<'runic> Runic<'runic> {
    pub fn describe(&self) -> OpenRunic {
        let timestamp = self.timestamp.unwrap_or_else(|| {
            10
        });

        let total = Describe::with(
            self.script,
            timestamp
        )
            .unwrap_or(0);

        OpenRunic::new(total)
    }
}

impl Default for Runic<'_> {
    fn default() -> Self {
        Self {
            script: "",
            timestamp: None,
            utc_offset: None,
        }
    }
}
