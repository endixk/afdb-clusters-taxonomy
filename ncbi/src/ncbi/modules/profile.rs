use std::error::Error;
use std::fs::File;
use std::io::{BufRead, Write, BufReader, BufWriter};

use crate::ncbi::entry_mapper::Entries;
use crate::ncbi::taxonomy_tree::Tree;
use crate::ncbi::modules::report::update_dfs;

pub fn read(input_path: String) -> Result<Vec<u32>, Box<dyn Error>> {
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);

    let mut ret = Vec::new();
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let x = line.trim().parse::<u32>()?;
        ret.push(x);
        line.clear();
    }
    Ok(ret)
}

// compile a boolean vector into valid characters, 7 bits per character
fn compile(ex: Vec<bool>) -> String {
    let mut ret = String::new();
    let mut cur = 0;
    let mut cnt = 0;
    for &x in &ex {
        cur |= (x as u8) << cnt;
        cnt += 1;
        if cnt == 7 {
            ret.push((cur + 33) as char);
            cur = 0;
            cnt = 0;
        }
    }
    if cnt > 0 {
        ret.push((cur + 33) as char);
    }
    ret
}
pub fn run(tree: &Tree, entries: &Entries, input_path: String, output_path: String) -> Result<(), Box<dyn Error>> {
    let ids = read(input_path)?;
    let n = ids.len();

    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    for (i, id) in ids.into_iter().enumerate() {
        if !tree.is_valid(id) {
            eprintln!("\rWarning: taxid {} is not valid", id);
            continue;
        }

        let name = if let Some(name) = tree.get_name(id) {
            name
        } else {
            // should not happen
            eprintln!("\rWarning: unable to get name of taxid {}", id);
            continue;
        };

        print!("\rProcessing {}/{}: {:50}...", i + 1, n, name);
        let mut ex = vec![false; entries.clu_size()];
        update_dfs(&mut ex, &tree, &entries, id);
        let ex = compile(ex);
        writeln!(writer, "{}|{}|{}", id, name, ex)?;
    }
    println!();
    Ok(())
}