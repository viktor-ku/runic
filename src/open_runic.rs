use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct OpenRunic {
    /// Total number of seconds computed from the runic
    total: u64,
}

#[wasm_bindgen]
impl OpenRunic {
    #[inline]
    pub(crate) fn new(total: u64) -> Self {
        Self { total }
    }

    #[inline]
    #[cfg(target_arch = "wasm32")]
    pub fn total(&self) -> f64 {
        self.total as _
    }

    #[inline]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn total(&self) -> u64 {
        self.total
    }
}
