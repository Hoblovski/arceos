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
        RNGST = RNGST.overflowing_mul(1289047261).0 + 121233897;
        RNGST
    }
}

pub fn randchr_ident() -> char {
    const IDENT: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
    IDENT[rand_usize() % IDENT.len()] as char
}

pub fn randchr_lower() -> char {
    const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    LOWER[rand_usize() % LOWER.len()] as char
}

/// Generate random string whose length is less than `l`, and whose characters are supplied by `f`.
pub fn rand_str<F: Fn() -> char>(f: F, l: usize) -> String {
    let mut s = String::new();
    let l = 1 + (rand_usize() % (l-1));
    for _ in 0..l {
        s.push(f());
    }
    s
}