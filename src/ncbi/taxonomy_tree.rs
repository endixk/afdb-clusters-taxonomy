extern crate fxhash;
use fxhash::FxHashMap;

pub struct Cargo {
    pub level: String,
    pub name: String,
}
pub struct Node {
    id: usize,
    parent: Option<usize>,
    cargo: Cargo,
}
pub struct Tree {
    nodes: Vec<Node>,
    map: FxHashMap<u32, usize>,
    iter: usize,
    root: usize,
}