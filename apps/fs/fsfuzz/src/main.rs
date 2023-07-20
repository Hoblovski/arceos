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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FuzzInputEntryType {
    Cwd,
    Cd,
    Mkdir,
    Rmdir,
    Rm,
    GetMetadata,
    Open,
    Create,
    ReadFile,
    WriteFile,
    CloseFile,
}

impl FuzzInputEntryType {
    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    pub fn from_usize(x: usize) -> Self {
        match x {
            0 => Self::Cwd,
            1 => Self::Cd,
            2 => Self::Mkdir,
            3 => Self::Rmdir,
            4 => Self::Rm,
            5 => Self::GetMetadata,
            6 => Self::Open,
            7 => Self::Create,
            8 => Self::ReadFile,
            9 => Self::WriteFile,
            10 => Self::CloseFile,
            _ => panic!("Invalid FuzzInputEntryType"),
        }
    }

    fn weight(&self) -> usize {
        match self {
            Self::Cwd => 2,
            Self::Cd => 5,
            Self::Mkdir => 0,
            Self::Rmdir => 0,
            Self::Rm => 0,
            Self::GetMetadata => 5,
            Self::Open => 0,
            Self::Create => 0,
            Self::ReadFile => 0,
            Self::WriteFile => 0,
            Self::CloseFile => 0,
        }
    }

    pub fn rand_by_weight() -> Self {
        const CANDIDATES: &[FuzzInputEntryType] = &[
            FuzzInputEntryType::Cwd,
            FuzzInputEntryType::Cd,
            FuzzInputEntryType::Mkdir,
            FuzzInputEntryType::Rmdir,
            FuzzInputEntryType::Rm,
            FuzzInputEntryType::GetMetadata,
            FuzzInputEntryType::Open,
            FuzzInputEntryType::Create,
            FuzzInputEntryType::ReadFile,
            FuzzInputEntryType::WriteFile,
            FuzzInputEntryType::CloseFile,
        ];
        let mut weight_presum = vec![0];
        weight_presum.extend(CANDIDATES.iter()
            .map(|x| x.weight())
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            }));
        let rand = rand_usize() % weight_presum.last().unwrap();
        let idx = weight_presum.iter().position(|x| rand < *x).unwrap();
        return CANDIDATES[idx - 1];
    }
}

struct Fuzzer {
    root: Rc<Node>,
    nsteps: usize,
}

impl Fuzzer {
    fn new() -> Self {
        Self {
            root: make_root_node("/"),
            nsteps: 0
        }
    }

    fn step(&mut self) {
        self.nsteps += 1;
        if self.nsteps & (0x400 - 1) == 0 {
            // Each 1024 steps: heartbeat
            println!("[fsfuzz] {} steps", self.nsteps);
        }
        match FuzzInputEntryType::rand_by_weight() {
            FuzzInputEntryType::Cwd => self.step_cwd(),
            FuzzInputEntryType::Cd => self.step_cd(),
            FuzzInputEntryType::Mkdir => self.step_mkdir(),
            FuzzInputEntryType::Rmdir => self.step_rmdir(),
            FuzzInputEntryType::Rm => self.step_rm(),
            FuzzInputEntryType::GetMetadata => self.step_getmetadata(),
            FuzzInputEntryType::Open => self.step_open(),
            FuzzInputEntryType::Create => self.step_create(),
            FuzzInputEntryType::ReadFile => self.step_readfile(),
            FuzzInputEntryType::WriteFile => self.step_writefile(),
            FuzzInputEntryType::CloseFile => self.step_closefile(),
        }
    }

    fn step_cwd(&self) {
        warn!("[fsfuzz] cwd");
        // cwd is not expected to fail
        libax::env::current_dir().unwrap();
    }

    fn step_cd(&self) {
        let p = self.root.rand_path();
        let p = p.as_str();
        warn!("[fsfuzz] cd {p}");
        libax::env::set_current_dir(&p);
    }

    fn step_mkdir(&self) {
        todo!()
    }

    fn step_rmdir(&self) {
        todo!()
    }

    fn step_rm(&self) {
        todo!()
    }

    fn step_getmetadata(&self) {
        let p = self.root.rand_path();
        let p = p.as_str();
        warn!("[fsfuzz] getmetadata {p}");
        fs::metadata(&p);
    }

    fn step_open(&self) {
        todo!()
    }

    fn step_create(&self) {
        todo!()
    }

    fn step_readfile(&self) {
        todo!()
    }

    fn step_writefile(&self) {
        todo!()
    }

    fn step_closefile(&self) {
        todo!()
    }
}

#[no_mangle]
fn main() {
    mainloop();
}

fn mainloop() {
    let mut fuzzer = Fuzzer::new();
    loop {
        fuzzer.step();
    }
}
