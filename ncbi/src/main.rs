mod ncbi;

use std::error::Error;
use clap::Parser;
use ncbi::*;

const NODE_PATH: &'static str = "lib/nodes.dmp";
const NAME_PATH: &'static str = "lib/names.dmp";
const MAP_PATH: &'static str = "lib/cluster.tsv";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Module to run [help, report, profile]
    module: String,

    /// Input file
    #[clap(short, long, default_value = "input.txt")]
    input: String,

    /// Output file
    #[clap(short, long, default_value = "output.txt")]
    output: String,
}

fn help() {
    println!("Usage: ncbi [module]");
    println!("Modules:");
    println!("  help    - Display this message");
    println!("  report  - Print a report");
    println!("  profile - Generate a profile");
    println!("    --input  - Input file (taxID)");
    println!("    --output - Output file (profile)");
    println!("  compare - Pairwise comparison of profiles");
    println!("    --input  - Input file (taxID)");
    println!("    --output - Output file (tsv)");
}

fn report() -> Result<(), Box<dyn Error>>{
    init::prepare()?;

    sanity::check::check(NODE_PATH.to_string())?;
    let mut tree = taxonomy_tree::build(NODE_PATH.to_string())?;
    taxonomy_tree::add_name(&mut tree, NAME_PATH.to_string())?;
    let entries = entry_mapper::map(MAP_PATH.to_string())?;

    modules::report::run(&tree, &entries)?;

    Ok(())
}

fn profile(args: Args) -> Result<(), Box<dyn Error>> {
    init::check_input(&args.input)?;
    init::check_output(&args.output)?;
    init::prepare()?;

    sanity::check::check(NODE_PATH.to_string())?;
    let mut tree = taxonomy_tree::build(NODE_PATH.to_string())?;
    taxonomy_tree::add_name(&mut tree, NAME_PATH.to_string())?;
    let entries = entry_mapper::map(MAP_PATH.to_string())?;

    modules::profile::run(&tree, &entries, args.input, args.output)?;

    Ok(())
}

fn compare(args: Args) -> Result<(), Box<dyn Error>> {
    init::check_input(&args.input)?;
    init::check_output(&args.output)?;
    init::prepare()?;

    sanity::check::check(NODE_PATH.to_string())?;
    let mut tree = taxonomy_tree::build(NODE_PATH.to_string())?;
    taxonomy_tree::add_name(&mut tree, NAME_PATH.to_string())?;
    let entries = entry_mapper::map(MAP_PATH.to_string())?;

    modules::compare::run(&tree, &entries, args.input, args.output)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.module.as_str() {
        "help" => help(),
        "report" => report()?,
        "profile" => profile(args)?,
        "compare" => compare(args)?,
        _ => help(),
    }

    Ok(())
}
