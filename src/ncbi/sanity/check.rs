use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn extract_rel(dump: Vec<String>) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let mut rel = Vec::new();
    for line in dump {
        let mut iter = line.split_whitespace();
        let id = iter.next().unwrap().parse::<usize>()?;
        let parent = iter.next().unwrap().parse::<usize>()?;
        rel.push((id, parent));
    }
    Ok(rel)
}

pub fn check(node_path: String) -> Result<(), Box<dyn Error>> {
    let node_file = File::open(node_path)?;
    let mut reader = BufReader::new(node_file);

    let mut dump = Vec::new();
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        dump.push(line.clone());
        line.clear();
    }

    let rel = extract_rel(dump)?;
    for (v, u) in rel {
        println!("{} {}", v, u);
    }

    Ok(())
}