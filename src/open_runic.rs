#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct OpenRunic {
    /// Total number of seconds computed from the runic
    total: u64,
}

impl OpenRunic {
    pub(crate) fn new(total: u64) -> Self {
        Self { total }
    }

    pub fn total(&self) -> u64 {
        self.total
    }
}
