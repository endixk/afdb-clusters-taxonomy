mod ncbi;
use ncbi::{entry_mapper, taxonomy_tree};

use std::env;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_path = args[1].clone();
    let name_path = args[2].clone();
    let mut tree = taxonomy_tree::build(node_path).unwrap();
    taxonomy_tree::add_name(&mut tree, name_path).unwrap();

    let map_path = args[3].clone();
    let mut entries = entry_mapper::map(map_path).unwrap();
}
