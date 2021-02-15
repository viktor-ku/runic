use crate::c::{SECOND, MINUTE, HOUR};
use crate::describe::Describe;
use crate::OpenRunic;

#[derive(Debug, PartialEq)]
pub struct Runic<'runic> {
    pub script: &'runic str,
    pub timestamp: Option<u64>,
    pub utc_offset: Option<u64>,
}

impl<'runic> Runic<'runic> {
    pub fn describe(&self) -> OpenRunic {
        todo!()
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
