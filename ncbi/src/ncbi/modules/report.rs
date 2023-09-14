use std::error::Error;
use std::io::{BufRead, BufReader, Write};

use crate::ncbi::entry_mapper::Entries;
use crate::ncbi::taxonomy_tree::{self, Tree};

fn update_dfs(ex: &mut [bool], tree: &Tree, entries: &Entries, xid: u32) {
    if let Some(v) = entries.get_clu(xid) {
        for &clu in &v {
            ex[clu] = true;
        }
    }
    for ch in tree.get_children(xid) {
        update_dfs(ex, tree, entries, ch);
    }
}
pub fn run(tree: &Tree, entries: &Entries) -> Result<(), Box<dyn Error>> {
    let mut si = BufReader::new(std::io::stdin().lock());
    loop {
        let mut s = String::new();
        print!("Enter taxid: ");
        std::io::stdout().flush()?;
        si.read_line(&mut s)?;
        let x = s.trim().parse::<u32>().unwrap_or_else(|_| {
            println!("Invalid taxid");
            u32::MAX
        });
        if x == 0 { break; }
        if x == u32::MAX { continue; }
        if !taxonomy_tree::report(&tree, x) { continue; }

        let mut ex = vec![false; entries.clu_size()];
        update_dfs(&mut ex, &tree, &entries, x);
        println!("-- Cluster entries --");
        let x = ex.iter().filter(|&&x| x).count();
        println!("Cluster count: {}", x);
        let print = loop {
            print!("Show cluster names? (y/n): ");
            std::io::stdout().flush().unwrap();
            let mut s = String::new();
            si.read_line(&mut s).unwrap();
            let s = s.trim();
            if s == "y" || s == "n" {
                break s == "y";
            } else {
                println!("Invalid input");
            }
        };
        if print {
            for i in 0..entries.clu_size() {
                if ex[i] {
                    print!("{} ", entries.clu_name(i));
                }
            }
            println!();
        }
        println!();
    }

    Ok(())
}