use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn extract_rel(dump: Vec<String>) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let mut rel = Vec::new();
    for line in dump {
        let mut iter = line.split_whitespace();
        let id = iter.next().unwrap().parse::<usize>()?;
        let par = iter.skip(1).next().unwrap().parse::<usize>()?;
        rel.push((id, par));
    }
    Ok(rel)
}

fn dfs(adj: &Vec<Vec<usize>>, vis: &mut Vec<bool>, id: usize) -> Result<(), Box<dyn Error>> {
    if vis[id] {
        return Err("Cycle detected".into());
    }
    vis[id] = true;
    for &ch in &adj[id] {
        dfs(adj, vis, ch)?;
    }
    Ok(())
}

pub fn check(node_path: String) -> Result<(), Box<dyn Error>> {
    println!("Checking tree...");
    let node_file = File::open(node_path)?;
    let mut reader = BufReader::new(node_file);

    let mut dump = Vec::new();
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        dump.push(line.clone());
        line.clear();
    }

    let rel = extract_rel(dump)?;
    let imax = *rel.iter().map(|(id, _)| id).max().unwrap();

    let mut ex = vec![false; imax + 1];
    let mut adj = vec![Vec::new(); imax + 1];
    for (id, par) in rel {
        if id == par {
            if id != 1 {
                return Err(format!("Non-root node {} is its own parent", id).into());
            } else {
                continue;
            }
        }
        adj[par].push(id);
        ex[id] = true;
        ex[par] = true;
    }

    let mut vis = vec![false; imax + 1];
    dfs(&adj, &mut vis, 1)?;
    for i in 1..=imax {
        if !vis[i] && ex[i] {
            println!("Node {} is unreachable", i);
        }
    }

    Ok(())
}