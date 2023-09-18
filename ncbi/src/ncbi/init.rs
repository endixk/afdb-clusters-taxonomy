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

pub fn check_input(input_path: &String) -> Result<(), Box<dyn Error>> {
    // check if input file exists
    let input_path = std::path::Path::new(input_path);
    if input_path.is_dir() {
        return Err(format!("Input file {} is a directory", input_path.to_str().unwrap()).into());
    }
    if !input_path.exists() {
        return Err(format!("Input file {} does not exist", input_path.to_str().unwrap()).into());
    }
    Ok(())
}

pub fn check_output(output_path: &String) -> Result<(), Box<dyn Error>> {
    // check if output file exists
    let output_path = std::path::Path::new(output_path);
    if output_path.is_dir() {
        return Err(format!("Output file {} is a directory", output_path.to_str().unwrap()).into());
    }
    if output_path.exists() {
        return Err(format!("Output file {} already exists", output_path.to_str().unwrap()).into());
    }

    // check if output file is writable
    let output_dir = output_path.parent().unwrap();
    let md = std::fs::metadata(output_dir)?;
    if md.permissions().readonly() {
        return Err(format!("Output directory {} is not writable", output_dir.to_str().unwrap()).into());
    }

    Ok(())
}