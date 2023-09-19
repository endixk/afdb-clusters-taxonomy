use std::error::Error;
use std::fs::File;
use std::io::{Write, BufWriter};

use crate::ncbi::entry_mapper::Entries;
use crate::ncbi::taxonomy_tree::Tree;
use crate::ncbi::modules::profile::read;
use crate::ncbi::modules::report::update_dfs;

fn _xor_pct_similarity(ex1: &[bool], ex2: &[bool], n: usize) -> Result<f64, Box<dyn Error>> {
    let mut cnt = n;
    for i in 0..n {
        if ex1[i] ^ ex2[i] { cnt -= 1; }
    }
    Ok(cnt as f64 * 100.0 / n as f64)
}
fn xor_union_similarity(ex1: &[bool], ex2: &[bool], n: usize) -> Result<f64, Box<dyn Error>> {
    let (mut x, mut c) = (0, 0);
    for i in 0..n {
        if ex1[i] || ex2[i] {
            c += 1;
            if !(ex1[i] ^ ex2[i]) { x += 1; }
        }
    }
    if c == 0 { c = 1; }
    Ok(x as f64 * 100.0 / c as f64)
}
pub fn run(tree: &Tree, entries: &Entries, input_path: String, output_path: String) -> Result<(), Box<dyn Error>> {
    let ids = read(input_path)?;
    let names = ids.iter().map(|&id| tree.get_name(id).unwrap()).collect::<Vec<_>>();
    let n = ids.len();
    let mut dmat = vec![vec![0.0; n]; n];

    let mut exv = Vec::new();
    for _ in 0..n { exv.push(vec![false; entries.clu_size()]); }
    ids.iter().enumerate().for_each(|(i, &id)| {
        print!("\rGenerating existence profiles... [{}/{}]", i + 1, n);
        update_dfs(&mut exv[i], &tree, &entries, id)
    });
    println!();

    for i in 0..n {
        print!("\rCalculating similarities... [{}/{}]", i + 1, n);
        for j in i..n {
            if i == j { dmat[i][j] = 100.0; }
            else {
                // dmat[i][j] = xor_pct_similarity(&exv[i], &exv[j], entries.clu_size())?;
                dmat[i][j] = xor_union_similarity(&exv[i], &exv[j], entries.clu_size())?;
                dmat[j][i] = dmat[i][j];
            }
        }
    }
    println!();

    println!("Writing output...");
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    for i in 0..n {
        for j in 0..n {
            writeln!(writer, "{}\t{}\t{}\t{}\t{}", ids[i], ids[j], names[i], names[j], dmat[i][j])?;
        }
    }

    Ok(())
}