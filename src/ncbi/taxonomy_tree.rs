extern crate fxhash;
use fxhash::{FxHashMap, FxHashSet};
use std::error::Error;

#[derive(Default)]
pub struct Cargo {
    pub level: Option<String>,
    pub name: Option<String>,
}

pub struct Node {
    id: u32,
    parent: Option<usize>,
    children: FxHashSet<usize>,
    cargo: Cargo,
}
impl Node {
    fn new(id: u32) -> Self {
        Node {
            id,
            parent: None,
            children: FxHashSet::default(),
            cargo: Cargo::default(),
        }
    }
    fn add_parent(&mut self, parent: usize) -> Result<(), Box<dyn Error>>{
        if self.parent.is_some() {
            return Err(format!("Node {} already has a parent", self.id).into());
        }
        self.parent = Some(parent);
        Ok(())
    }
    fn add_child(&mut self, child: usize) -> Result<(), Box<dyn Error>>{
        if self.children.contains(&child) {
            return Err(format!("Node {} already has child {}", self.id, child).into());
        }
        self.children.insert(child);
        Ok(())
    }
    fn add_level(&mut self, level: String) -> Result<(), Box<dyn Error>>{
        if self.cargo.level.is_some() {
            return Err(format!("Node {} already has a level", self.id).into());
        }
        self.cargo.level = Some(level);
        Ok(())
    }
    fn add_name(&mut self, name: String) -> Result<(), Box<dyn Error>>{
        if self.cargo.name.is_some() {
            return Err(format!("Node {} already has a name", self.id).into());
        }
        self.cargo.name = Some(name);
        Ok(())
    }
}

pub struct Tree {
    nodes: Vec<Node>,
    map: FxHashMap<u32, usize>,
    iter: usize,
    root: Option<usize>,
    name_map: FxHashMap<String, usize>,
}
impl Tree {
    fn new() -> Self {
        Tree {
            nodes: Vec::new(),
            map: FxHashMap::default(),
            iter: 0,
            root: None,
            name_map: FxHashMap::default(),
        }
    }
    fn add_rel(&mut self, id: u32, par: u32, rank: String) -> Result<(), Box<dyn Error>> {
        let id = if self.map.contains_key(&id) {
            *self.map.get(&id).unwrap()
        } else {
            self.nodes.push(Node::new(id));
            self.map.insert(id, self.iter);
            self.iter += 1;
            self.iter - 1
        };
        let par = if self.map.contains_key(&par) {
            *self.map.get(&par).unwrap()
        } else {
            self.nodes.push(Node::new(par));
            self.map.insert(par, self.iter);
            self.iter += 1;
            self.iter - 1
        };

        // self parent
        if id == par {
            return if self.root.is_some() {
                Err(format!("Node {} is self parent but root is already set to {}", id, self.root.unwrap()).into())
            } else {
                self.root = Some(id);
                self.nodes[id].add_level(rank)?;
                Ok(())
            }
        }

        // add relation
        self.nodes[id].add_parent(par)?;
        self.nodes[par].add_child(id)?;
        self.nodes[id].add_level(rank)?;

        Ok(())
    }
    fn add_name(&mut self, id: u32, name: String) -> Result<(), Box<dyn Error>> {
        let id = if self.map.contains_key(&id) {
            *self.map.get(&id).unwrap()
        } else {
            return Err(format!("Node {} does not exist", id).into());
        };
        self.nodes[id].add_name(name.clone())?;
        self.name_map.insert(name.to_lowercase(), id);
        Ok(())
    }

    pub fn get_children(&self, id: u32) -> Vec<u32> {
        let id = *self.map.get(&id).unwrap();
        self.nodes[id].children.iter().map(|&x| self.nodes[x].id).collect()
    }
}

use std::io::{BufRead, BufReader};
use std::fs::File;
pub fn dump(path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut dump = Vec::new();
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        dump.push(line.clone());
        line.clear();
    }
    Ok(dump)
}
fn extract_rel(dump: Vec<String>) -> Result<Vec<(u32, u32, String)>, Box<dyn Error>> {
    let mut ret = Vec::new();
    for line in dump {
        let mut iter = line.split('|');
        let id = iter.next().unwrap().trim().parse::<u32>()?;
        let par = iter.next().unwrap().trim().parse::<u32>()?;
        let rank = iter.next().unwrap().trim().to_string();
        ret.push((id, par, rank));
    }
    Ok(ret)
}
fn extract_name(dump: Vec<String>) -> Result<Vec<(u32, String)>, Box<dyn Error>> {
    let mut ret = Vec::new();
    for line in dump {
        let mut iter = line.split('|');
        let id = iter.next().unwrap().trim().parse::<u32>()?;
        let name = iter.next().unwrap().trim().to_string();
        ret.push((id, name));
    }
    Ok(ret)
}
pub fn build(node_path: String) -> Result<Tree, Box<dyn Error>> {
    println!("Building tree...");
    let rel = extract_rel(dump(node_path)?)?;
    let mut tree = Tree::new();
    for (id, par, rank) in rel {
        tree.add_rel(id, par, rank)?;
    }
    Ok(tree)
}
pub fn add_name(tree: &mut Tree, name_path: String) -> Result<(), Box<dyn Error>> {
    println!("Adding names...");
    let names = extract_name(dump(name_path)?)?;
    for (id, name) in names {
        tree.add_name(id, name)?;
    }
    Ok(())
}

pub fn report(tree: &Tree, id: u32) {
    let xid = tree.map.get(&id);
    if let None = xid {
        println!("taxID {} does not exist", id);
        return;
    }
    let mut id = *xid.unwrap();

    println!("--- Node info ---");
    let node = &tree.nodes[id];
    println!("ID    : {}", node.id);
    println!("Name  : {}", node.cargo.name.as_ref().unwrap());
    println!("Level : {}", node.cargo.level.as_ref().unwrap());
    println!();

    println!("--- Children ---");
    for &ch in &node.children {
        let ch = &tree.nodes[ch];
        println!("{:10} {:16} {}", ch.id, ch.cargo.level.as_ref().unwrap(), ch.cargo.name.as_ref().unwrap());
    }
    println!();

    println!("--- Ancestors ---");
    let mut st = Vec::new();
    loop {
        st.push(id);
        if id == tree.root.unwrap() {
            break;
        }
        id = tree.nodes[id].parent.unwrap();
    }
    while let Some(id) = st.pop() {
        let node = &tree.nodes[id];
        println!("{:10} {:16} {}", node.id, node.cargo.level.as_ref().unwrap(), node.cargo.name.as_ref().unwrap());
    }
    println!();
}