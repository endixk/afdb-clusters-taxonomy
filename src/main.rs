mod ncbi;
use ncbi::taxonomy_tree;

use std::env;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_path = args[1].clone();
    let name_path = args[2].clone();
    let mut tree = taxonomy_tree::build(node_path).unwrap();
    taxonomy_tree::add_name(&mut tree, name_path).unwrap();

    let mut si = BufReader::new(std::io::stdin().lock());
    let mut s = String::new();
    loop {
        print!("Enter taxid: ");
        std::io::stdout().flush().unwrap();
        si.read_line(&mut s).unwrap();
        let x = s.trim().parse::<u32>().unwrap();
        if x == 0 { break; }
        taxonomy_tree::report(&tree, x);
        s.clear();
    }
}
