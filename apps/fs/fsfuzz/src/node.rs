use crate::prelude::*;

/// In memory representation of any hierarchical system.
/// Used to generate sensible paths for fuzzing.
#[derive(Debug)]
pub struct Node {
    name: String,
    size: usize,
    chs: Vec<Rc<Node>>,
}

/// Represents a path between a node and one of its descendents.
#[derive(Debug)]
pub struct Path(Vec<String>);

impl Path {
    pub fn as_str(&self) -> String {
        self.0.join("/")
    }
}

impl Node {
    pub fn n0(name: &str) -> Rc<Node> {
        Rc::new(Node {
            name: name.to_string(),
            size: 1,
            chs: vec![],
        })
    }

    pub fn nn(name: &str, chs: Vec<Rc<Node>>) -> Rc<Node> {
        Rc::new(Node {
            name: name.to_string(),
            size: chs.iter().map(|ch| ch.size).sum::<usize>() + 1,
            chs,
        })
    }

    pub fn descend_path(self: &Rc<Self>, path: Path) -> Option<Rc<Node>> {
        let mut cur = self;
        for seg in path.0 {
            let chidx = cur.chs.iter().position(|ch| ch.name == seg)?;
            cur = &cur.chs[chidx];
        }
        Some(cur.clone())
    }

    fn descend_nth(self: &Rc<Self>, n: usize) -> Option<(Path, Rc<Node>)> {
        if n >= self.size {
            return None;
        }
        let mut cur = self;
        let mut n = n;
        let mut path = vec![];
        loop {
            path.push(cur.name.clone());
            if n == 0 {
                return Some((Path(path), cur.clone()));
            }
            n -= 1;
            for ch in &cur.chs {
                if ch.size <= n {
                    n -= ch.size;
                } else {
                    cur = ch;
                    break;
                }
            }
        }
    }

    /// Select a random but valid path leading to some descendent.
    pub fn rand_path(self: &Rc<Self>) -> Path {
        let n = rand_usize() % self.size;
        let (path, _) = self.descend_nth(n).unwrap();
        path
    }

    pub fn dump(self: &Rc<Self>, indent: usize) {
        for _ in 0..indent {
            print!("  ");
        }
        println!("{}/", self.name);
        for ch in &self.chs {
            ch.dump(indent + 1);
        }
    }
}
