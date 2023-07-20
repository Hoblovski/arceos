#![no_std]
#![no_main]
#![allow(unused)]

mod prelude;

#[macro_use]
extern crate libax;
extern crate alloc;

use crate::prelude::*;

#[no_mangle]
fn main() {
    println!("Entries from `fs::read_dir(\"/a\")`");
    let ents = fs::read_dir("/a").unwrap();
    for ent in ents {
        println!("{:?}", ent);
    }

    println!("........");

    println!("File::open(\"/a/Am\")");
    match File::open("/a/Am") {
        Ok(_) => unreachable!(),
        Err(e) => println!("Open failed: {:?}", e),
    }
}
