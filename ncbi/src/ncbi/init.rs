extern crate reqwest;

use std::error::Error;
use std::fs::File;

fn download(local_path: String, remote_path: String) -> Result<(), Box<dyn Error>> {
    println!("Downloading {}...", local_path);
    let mut resp = reqwest::blocking::get(remote_path.as_str())?;
    let mut file = File::create(local_path.as_str())?;
    std::io::copy(&mut resp, &mut file)?;
    Ok(())
}
pub fn prepare() -> Result<(), Box<dyn Error>> {
    // check directory "lib"
    let lib_dir = std::path::Path::new("lib");
    if !lib_dir.exists() {
        std::fs::create_dir(lib_dir)?;
    }

    // check file "lib/nodes.dmp"
    let node_path = lib_dir.join("nodes.dmp");
    if !node_path.exists() {
        download(node_path.to_str().unwrap().to_string(), "http://147.47.218.69:9000/afdb-tax/nodes.dmp".to_string())?;
    }

    // check file "lib/names.dmp"
    let name_path = lib_dir.join("names.dmp");
    if !name_path.exists() {
        download(name_path.to_str().unwrap().to_string(), "http://147.47.218.69:9000/afdb-tax/snames.dmp".to_string())?;
    }

    // check file "lib/cluster.tsv"
    let map_path = lib_dir.join("cluster.tsv");
    if !map_path.exists() {
        download(map_path.to_str().unwrap().to_string(), "http://147.47.218.69:9000/afdb-tax/entry_rep_tax.tsv".to_string())?;
    }

    Ok(())
}