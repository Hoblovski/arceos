#![no_std]
#![no_main]
#![allow(unused)]

mod node;
mod prelude;

#[macro_use]
extern crate libax;
extern crate alloc;

use crate::node::Node;
use crate::prelude::*;

fn make_root_node(p: &str) -> Rc<Node> {
    libax::env::set_current_dir(p).unwrap();
    let name = p.split('/').last().unwrap();
    let mut chs = Vec::new();

    for ent in fs::read_dir(p).unwrap() {
        let ent = ent.unwrap();
        let path = ent.path();
        let ent_md = fs::metadata(&path).unwrap();
        if ent_md.is_file() {
            chs.push(Node::n0(&ent.file_name()));
        } else if ent_md.is_dir() {
            chs.push(make_root_node(&path));
        } else {
            // Ignored. Fuzz other file types in the future.
        }
    }
    Node::nn(name, chs)
}

#[no_mangle]
fn main() {
    mainloop();
}

fn mainloop() {
    let root = make_root_node("/");
    root.dump(0);

    loop {}
}
