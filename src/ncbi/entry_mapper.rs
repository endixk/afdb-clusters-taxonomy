extern crate fxhash;
use fxhash::{FxHashMap, FxHashSet};
use std::error::Error;

struct Clusters {
    names: Vec<String>,
    map: FxHashMap<String, usize>,
    iter: usize,
}
impl Clusters {
    fn new() -> Self {
        Clusters {
            names: Vec::new(),
            map: FxHashMap::default(),
            iter: 0,
        }
    }
    fn insert(&mut self, name: String) -> Result<usize, Box<dyn Error>> {
        if self.map.contains_key(&name) {
            return Err(format!("Cluster {} already exists", name).into());
        }
        self.names.push(name.clone());
        self.map.insert(name, self.iter);
        self.iter += 1;
        Ok(self.iter - 1)
    }
}

#[derive(Default)]
struct Entry {
    // id: usize,
    set: FxHashSet<usize>,
}
pub struct Entries {
    vec: Vec<Entry>,
    clu: Clusters,
    map: FxHashMap<u32, usize>,
    iter: usize,
}
impl Entries {
    fn new() -> Self {
        Entries {
            vec: Vec::new(),
            clu: Clusters::new(),
            map: FxHashMap::default(),
            iter: 0,
        }
    }
    fn insert(&mut self, id: u32) -> Result<usize, Box<dyn Error>> {
        if self.map.contains_key(&id) {
            return Err(format!("Entry {} already exists", id).into());
        }
        self.vec.push(Entry {
            // id: self.iter,
            set: FxHashSet::default(),
        });
        self.map.insert(id, self.iter);
        self.iter += 1;
        Ok(self.iter - 1)
    }
    fn add_clu(&mut self, id: u32, clu: String) -> Result<(), Box<dyn Error>> {
        let idx = if self.map.contains_key(&id) {
            *self.map.get(&id).unwrap()
        } else {
            self.insert(id)?
        };

        let clu = if self.clu.map.contains_key(&clu) {
            *self.clu.map.get(&clu).unwrap()
        } else {
            self.clu.insert(clu)?
        };

        self.vec[idx].set.insert(clu);
        Ok(())
    }

    pub fn clu_size(&self) -> usize {
        self.clu.iter
    }
    pub fn clu_name(&self, id: usize) -> String {
        self.clu.names[id].clone()
    }
    pub fn get_clu(&self, id: u32) -> Option<Vec<usize>> {
        let idx = self.map.get(&id)?;
        let mut ret = Vec::new();
        for &clu in &self.vec[*idx].set {
            ret.push(clu);
        }
        Some(ret)
    }
}

use crate::taxonomy_tree;
fn extract(dump: Vec<String>) -> Result<Vec<(String, String, u32)>, Box<dyn Error>> {
    let mut ret = Vec::new();
    for line in dump {
        let mut iter = line.split_ascii_whitespace();
        let mem = iter.next().unwrap().trim().to_string();
        let rep = iter.next().unwrap().trim().to_string();
        let xid = iter.next().unwrap().trim().parse::<u32>()?;
        ret.push((mem, rep, xid));
    }
    Ok(ret)
}
pub fn map(map_path: String) -> Result<Entries, Box<dyn Error>> {
    println!("Mapping entries...");
    let dump = taxonomy_tree::dump(map_path)?;
    let rel = extract(dump)?;

    let mut entries = Entries::new();
    for (_, clu, xid) in rel {
        entries.add_clu(xid, clu)?;
    }
    Ok(entries)
}