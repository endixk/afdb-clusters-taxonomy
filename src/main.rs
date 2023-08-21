mod ncbi;
use ncbi::sanity;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_path = args[1].clone();
    sanity::check::check(node_path).unwrap();
}
