#[derive(PartialEq)]
pub struct OpenRunic {
    /// Total number of seconds computed from the runic
    total: u64,
}

impl OpenRunic {
    #[inline]
    pub fn total(&self) -> u64 {
        self.total
    }

    #[inline]
    pub(crate) fn new(total: u64) -> Self {
        Self { total }
    }
}
