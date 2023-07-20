pub use alloc::rc::Rc;
pub use alloc::collections::BTreeMap;

pub use libax::fs::{self, File};
pub use libax::io::{self, prelude::*};
pub use libax::string::ToString;
pub use libax::{string::String, vec::Vec};

/// SAFETY: this function is unsafe and racy but we dont care.
pub fn rand_usize() -> usize {
    static mut RNGST: usize = 0;
    unsafe {
        RNGST = RNGST.overflowing_mul(100000007).0 + 123897;
        RNGST
    }
}
