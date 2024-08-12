// rrsignal.rs

use super::common;
use common::EdgeEnd;

pub struct RRsignal {
    pub is_red: bool,
    pub edge:   EdgeEnd,
}

impl RRsignal {
    pub fn signal_is_red(&self) -> bool {
        self.is_red
    }
}